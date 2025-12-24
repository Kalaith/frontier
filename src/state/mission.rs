//! Mission state - expedition flow with events and encounters

use macroquad::prelude::*;
use crate::missions::{Mission, NodeType, MapNode};
use crate::kingdom::PartyMemberState;
use super::{StateTransition, ResultState};
use super::combat::{CombatState, MissionContext};

/// Active mission/expedition state with branching paths
pub struct MissionState {
    pub mission: Mission,
    /// Current node ID in the map graph
    pub current_node_id: usize,
    /// All party members on this mission (first is leader)
    pub party_members: Vec<PartyMemberState>,
    /// Generated branching map for this mission
    pub map_nodes: Vec<MapNode>,
    /// Nodes that have been visited (by ID)
    pub visited_nodes: Vec<usize>,
    /// If at a fork, which path options are available
    pub available_paths: Vec<usize>,
    /// Selected path index when at a fork
    pub selected_path: usize,
}

impl Default for MissionState {
    fn default() -> Self {
        let mission = Mission::first_mission();
        let map_nodes = mission.generate_branching_map();
        Self {
            mission,
            current_node_id: 0,
            party_members: vec![],
            map_nodes,
            visited_nodes: vec![0],
            available_paths: vec![],
            selected_path: 0,
        }
    }
}

impl MissionState {
    /// Get the party leader
    pub fn leader(&self) -> Option<&PartyMemberState> {
        self.party_members.first()
    }
    
    /// Get mutable party leader
    #[allow(dead_code)]
    pub fn leader_mut(&mut self) -> Option<&mut PartyMemberState> {
        self.party_members.first_mut()
    }
    
    /// Create from a Mission object with a full party
    pub fn from_mission_with_party(mission: Mission, party_members: Vec<PartyMemberState>) -> Self {
        let map_nodes = mission.generate_branching_map();
        Self {
            mission,
            current_node_id: 0,
            party_members,
            map_nodes,
            visited_nodes: vec![0],
            available_paths: vec![],
            selected_path: 0,
        }
    }
    
    /// Set the current node (used when returning from combat)
    pub fn with_node(mut self, node: usize) -> Self {
        self.current_node_id = node;
        self
    }
    
    /// Set map nodes (used when returning from combat to preserve the generated layout)
    pub fn with_map_nodes(mut self, map_nodes: Vec<MapNode>) -> Self {
        self.map_nodes = map_nodes;
        self
    }
    
    /// Set visited nodes (used when returning from combat)
    pub fn with_visited(mut self, visited: Vec<usize>) -> Self {
        self.visited_nodes = visited;
        self
    }
    
    /// Get the current map node
    fn current_node(&self) -> Option<&MapNode> {
        self.map_nodes.iter().find(|n| n.id == self.current_node_id)
    }
    
    /// Check if mission is complete (visited the final node)
    fn is_complete(&self) -> bool {
        if let Some(node) = self.current_node() {
            node.connections.is_empty()
        } else {
            false
        }
    }
    
    /// Get node screen position for mouse hit testing (x, y, size)
    fn get_node_screen_pos(&self, node_id: usize) -> Option<(f32, f32, f32)> {
        let node = self.map_nodes.iter().find(|n| n.id == node_id)?;
        
        let map_y = 140.0;
        let node_size = 50.0;
        let layer_gap = 100.0;
        let node_gap = 70.0;
        let start_x = 50.0;
        
        let node_x = start_x + (node.layer as f32 * layer_gap);
        
        let nodes_in_layer: Vec<_> = self.map_nodes.iter()
            .filter(|n| n.layer == node.layer)
            .collect();
        let layer_height = (nodes_in_layer.len() as f32 - 1.0) * node_gap;
        let layer_start_y = map_y + (150.0 - layer_height) / 2.0;
        let node_y = layer_start_y + (node.position as f32 * node_gap);
        
        Some((node_x, node_y, node_size))
    }
    
    /// Process the current node's encounter
    fn process_current_node(&mut self) -> Option<StateTransition> {
        let node = self.current_node()?.clone();  // Clone to avoid borrow issues
        
        match &node.node_type {
            NodeType::Combat | NodeType::Boss => {
                // Create combat with the full party
                let context = MissionContext {
                    mission: self.mission.clone(),
                    current_node: self.current_node_id,
                    party_members: self.party_members.clone(),
                    map_nodes: self.map_nodes.clone(),
                    visited_nodes: self.visited_nodes.clone(),
                };
                let combat = CombatState::for_mission(context);
                return Some(StateTransition::ToCombat(combat));
            }
            NodeType::Event => {
                if let Some(event) = crate::missions::events::random_event(self.current_node_id, &self.mission.region_id) {
                    if let Some(leader) = self.leader() {
                        let event_state = super::EventState::new(
                            event,
                            leader.id.clone(),
                            leader.name.clone(),
                        ).with_mission_context(
                            self.mission.clone(),
                            self.current_node_id,
                            self.party_members.clone(),
                        );
                        return Some(StateTransition::ToEvent(event_state));
                    }
                }
            }
            NodeType::Rest => {
                // Rest nodes heal the party slightly and reduce stress
                for member in &mut self.party_members {
                    let heal = (member.max_hp as f32 * 0.1) as i32;
                    member.hp = (member.hp + heal).min(member.max_hp);
                    member.stress = (member.stress - 5).max(0);
                }
            }
        }
        None
    }
    
    pub fn update(&mut self) -> Option<StateTransition> {
        // Check if we have path options to choose from
        if !self.available_paths.is_empty() {
            // Path selection with arrow keys
            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                if self.selected_path > 0 {
                    self.selected_path -= 1;
                }
            }
            if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                if self.selected_path < self.available_paths.len().saturating_sub(1) {
                    self.selected_path += 1;
                }
            }
            
            // Number keys for quick selection
            for i in 0..self.available_paths.len().min(9) {
                let key = match i {
                    0 => KeyCode::Key1,
                    1 => KeyCode::Key2,
                    2 => KeyCode::Key3,
                    _ => continue,
                };
                if is_key_pressed(key) {
                    self.selected_path = i;
                }
            }
            
            // Mouse click on path nodes
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                for (idx, &node_id) in self.available_paths.iter().enumerate() {
                    if let Some((nx, ny, size)) = self.get_node_screen_pos(node_id) {
                        if mx >= nx && mx <= nx + size && my >= ny && my <= ny + size {
                            if self.selected_path == idx {
                                // Already selected - confirm
                                self.current_node_id = node_id;
                                self.visited_nodes.push(node_id);
                                self.available_paths.clear();
                                self.selected_path = 0;
                                
                                if let Some(transition) = self.process_current_node() {
                                    return Some(transition);
                                }
                                
                                if self.is_complete() {
                                    let mut results = ResultState::victory_for_party(&self.party_members);
                                    results.stress_gained = self.mission.base_stress;
                                    results.rewards = vec![
                                        format!("+{} Supplies", self.mission.reward_supplies),
                                        format!("+{} Knowledge", self.mission.reward_knowledge),
                                    ];
                                    return Some(StateTransition::ToResults(results));
                                }
                                break;
                            } else {
                                // Select this path
                                self.selected_path = idx;
                                break;
                            }
                        }
                    }
                }
            }
            
            // Confirm path with Space or Enter
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                if let Some(&next_node_id) = self.available_paths.get(self.selected_path) {
                    self.current_node_id = next_node_id;
                    self.visited_nodes.push(next_node_id);
                    self.available_paths.clear();
                    self.selected_path = 0;
                    
                    // Process the new node
                    if let Some(transition) = self.process_current_node() {
                        return Some(transition);
                    }
                    
                    // If mission complete after this node
                    if self.is_complete() {
                        let mut results = ResultState::victory_for_party(&self.party_members);
                        results.stress_gained = self.mission.base_stress;
                        results.rewards = vec![
                            format!("+{} Supplies", self.mission.reward_supplies),
                            format!("+{} Knowledge", self.mission.reward_knowledge),
                        ];
                        return Some(StateTransition::ToResults(results));
                    }
                }
            }
        } else {
            // Space to advance to next node(s)
            if is_key_pressed(KeyCode::Space) {
                if let Some(node) = self.current_node() {
                    let connections = node.connections.clone();
                    
                    if connections.is_empty() {
                        // Mission complete!
                        let mut results = ResultState::victory_for_party(&self.party_members);
                        results.stress_gained = self.mission.base_stress;
                        results.rewards = vec![
                            format!("+{} Supplies", self.mission.reward_supplies),
                            format!("+{} Knowledge", self.mission.reward_knowledge),
                        ];
                        return Some(StateTransition::ToResults(results));
                    } else if connections.len() == 1 {
                        // Only one path - auto-advance
                        self.current_node_id = connections[0];
                        self.visited_nodes.push(connections[0]);
                        
                        if let Some(transition) = self.process_current_node() {
                            return Some(transition);
                        }
                    } else {
                        // Multiple paths - show choice
                        self.available_paths = connections;
                        self.selected_path = 0;
                    }
                }
            }
        }
        
        // Escape to retreat
        if is_key_pressed(KeyCode::Escape) {
            return Some(StateTransition::ToBase);
        }
        
        None
    }
    
    pub fn draw(&self, textures: &std::collections::HashMap<String, Texture2D>) {
        // Draw background
        let bg_path = format!("assets/images/regions/{}.png", self.mission.region_id);
        if let Some(tex) = textures.get(&bg_path) {
            draw_texture_ex(
                tex,
                0.0, 0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                }
            );
            
            // Dark overlay for readability
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 150));
        }

        draw_text(&format!("MISSION: {}", self.mission.name), 20.0, 40.0, 28.0, WHITE);
        draw_text(&format!("{:?} Mission", self.mission.mission_type), 20.0, 70.0, 18.0, GRAY);
        
        // Show party members
        let party_label = if self.party_members.len() == 1 {
            format!("Adventurer: {}", self.leader().map(|m| m.name.as_str()).unwrap_or("?"))
        } else {
            format!("Party ({}):", self.party_members.len())
        };
        draw_text(&party_label, 20.0, 95.0, 20.0, SKYBLUE);
        
        // Draw party member portraits in a row
        let portrait_size = 60.0;
        let portrait_start_x = 650.0;
        
        for (i, member) in self.party_members.iter().enumerate() {
            let x = portrait_start_x + (i as f32 * (portrait_size + 5.0));
            
            // Portrait
            if let Some(path) = &member.image_path {
                if let Some(tex) = textures.get(path) {
                    let tint = if member.hp < member.max_hp / 3 { 
                        Color::from_rgba(255, 100, 100, 255) 
                    } else { 
                        WHITE 
                    };
                    draw_texture_ex(
                        tex,
                        x, 30.0,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(vec2(portrait_size, portrait_size)),
                            ..Default::default()
                        }
                    );
                }
            }
            
            // HP bar under portrait
            let hp_pct = member.hp as f32 / member.max_hp as f32;
            draw_rectangle(x, 95.0, portrait_size, 6.0, DARKGRAY);
            draw_rectangle(x, 95.0, portrait_size * hp_pct, 6.0, GREEN);
            
            // Leader star
            if i == 0 {
                draw_text("★", x + portrait_size - 12.0, 42.0, 14.0, YELLOW);
            }
        }
        
        // Draw branching map
        self.draw_branching_map();
        
        // Instructions
        if self.available_paths.is_empty() {
            draw_text("[SPACE] Advance  [ESC] Retreat", 20.0, screen_height() - 40.0, 20.0, GREEN);
        } else {
            draw_text("[←/→ or 1-3] Choose Path  [SPACE] Confirm  [ESC] Retreat", 20.0, screen_height() - 40.0, 20.0, YELLOW);
        }
    }
    
    /// Draw the branching map visualization
    fn draw_branching_map(&self) {
        let map_y = 140.0;
        let node_size = 50.0;
        let layer_gap = 100.0;
        let node_gap = 70.0;
        let start_x = 50.0;
        
        // Group nodes by layer
        let max_layer = self.map_nodes.iter().map(|n| n.layer).max().unwrap_or(0);
        
        // First pass: draw connections
        for node in &self.map_nodes {
            let node_x = start_x + (node.layer as f32 * layer_gap);
            
            // Get nodes in this layer to calculate Y position
            let nodes_in_layer: Vec<_> = self.map_nodes.iter()
                .filter(|n| n.layer == node.layer)
                .collect();
            let layer_height = (nodes_in_layer.len() as f32 - 1.0) * node_gap;
            let layer_start_y = map_y + (150.0 - layer_height) / 2.0;
            let node_y = layer_start_y + (node.position as f32 * node_gap);
            
            // Draw connections to next nodes
            for &target_id in &node.connections {
                if let Some(target) = self.map_nodes.iter().find(|n| n.id == target_id) {
                    let target_x = start_x + (target.layer as f32 * layer_gap);
                    let target_nodes_in_layer: Vec<_> = self.map_nodes.iter()
                        .filter(|n| n.layer == target.layer)
                        .collect();
                    let target_layer_height = (target_nodes_in_layer.len() as f32 - 1.0) * node_gap;
                    let target_layer_start_y = map_y + (150.0 - target_layer_height) / 2.0;
                    let target_y = target_layer_start_y + (target.position as f32 * node_gap);
                    
                    // Line color based on whether this is a selectable path
                    let line_color = if self.available_paths.contains(&target_id) {
                        if self.available_paths.get(self.selected_path) == Some(&target_id) {
                            YELLOW
                        } else {
                            Color::from_rgba(100, 150, 100, 255)
                        }
                    } else if self.visited_nodes.contains(&target_id) || self.visited_nodes.contains(&node.id) {
                        GREEN
                    } else {
                        DARKGRAY
                    };
                    
                    draw_line(
                        node_x + node_size / 2.0,
                        node_y + node_size / 2.0,
                        target_x + node_size / 2.0,
                        target_y + node_size / 2.0,
                        2.0,
                        line_color,
                    );
                }
            }
        }
        
        // Second pass: draw nodes
        for node in &self.map_nodes {
            let node_x = start_x + (node.layer as f32 * layer_gap);
            
            let nodes_in_layer: Vec<_> = self.map_nodes.iter()
                .filter(|n| n.layer == node.layer)
                .collect();
            let layer_height = (nodes_in_layer.len() as f32 - 1.0) * node_gap;
            let layer_start_y = map_y + (150.0 - layer_height) / 2.0;
            let node_y = layer_start_y + (node.position as f32 * node_gap);
            
            // Node color
            let (bg_color, border_color) = if node.id == self.current_node_id {
                (YELLOW, WHITE)
            } else if self.available_paths.contains(&node.id) {
                let is_selected = self.available_paths.get(self.selected_path) == Some(&node.id);
                if is_selected {
                    (Color::from_rgba(100, 200, 100, 255), YELLOW)
                } else {
                    (Color::from_rgba(80, 120, 80, 255), GREEN)
                }
            } else if self.visited_nodes.contains(&node.id) {
                (GREEN, WHITE)
            } else {
                (DARKGRAY, GRAY)
            };
            
            draw_rectangle(node_x, node_y, node_size, node_size, bg_color);
            draw_rectangle_lines(node_x, node_y, node_size, node_size, 2.0, border_color);
            
            // Node icon
            let (icon, icon_color) = match &node.node_type {
                NodeType::Combat => ("X", RED),
                NodeType::Boss => ("!", PURPLE),
                NodeType::Event => ("?", SKYBLUE),
                NodeType::Rest => ("+", GREEN),
            };
            let text_color = if self.visited_nodes.contains(&node.id) || node.id == self.current_node_id { 
                BLACK 
            } else { 
                icon_color 
            };
            draw_text(icon, node_x + 18.0, node_y + 35.0, 28.0, text_color);
            
            // Show selection number if path choice
            if let Some(idx) = self.available_paths.iter().position(|&id| id == node.id) {
                draw_text(&format!("[{}]", idx + 1), node_x + 15.0, node_y - 5.0, 16.0, YELLOW);
            }
        }
        
        // Progress indicator
        let current_layer = self.current_node().map(|n| n.layer).unwrap_or(0);
        let progress = format!("Layer {}/{}", current_layer + 1, max_layer + 1);
        draw_text(&progress, 20.0, 130.0, 20.0, YELLOW);
    }
}
