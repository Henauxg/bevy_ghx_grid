use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    ecs::{bundle::Bundle, schedule::IntoScheduleConfigs, system::Query},
    gizmos::AppGizmoBuilder,
    math::{Vec2, Vec3},
    transform::TransformSystem,
};
use ghx_grid::{
    cartesian::coordinates::{CartesianCoordinates, CartesianPosition},
    coordinate_system::CoordinateSystem,
};

use self::{
    markers::{
        despawn_debug_markers, draw_debug_markers_2d, draw_debug_markers_3d,
        insert_transform_on_new_markers, MarkerDespawnEvent, MarkersGroup,
    },
    view::{
        draw_debug_grids_2d, draw_debug_grids_3d, DebugGridView, DebugGridView2d, DebugGridView3d,
        GridViewGizmoGroup,
    },
};

/// Defines markers drawn as [bevy::prelude::Gizmos], useful for debugging & visualization
pub mod markers;
/// Components and systems to visualize 2d & 3d grids
pub mod view;

/// Bevy plugin used to visualize cartesian [`ghx_grid::grid::Grid`] and additional debug markers created with [`markers::MarkerDespawnEvent`].
pub struct GridDebugPlugin<C: CoordinateSystem> {
    typestate: PhantomData<C>,
}

impl<T: CoordinateSystem> GridDebugPlugin<T> {
    /// Create a new GridDebugPlugin
    pub fn new() -> Self {
        Self {
            typestate: PhantomData,
        }
    }
}

impl<C: CartesianCoordinates> Plugin for GridDebugPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_debug_grids_3d::<C>, draw_debug_grids_2d::<C>))
            .add_systems(
                PostUpdate,
                (
                    ((despawn_debug_markers, insert_transform_on_new_markers).chain())
                        .before(TransformSystem::TransformPropagate),
                    (draw_debug_markers_3d, draw_debug_markers_2d)
                        .after(TransformSystem::TransformPropagate),
                ),
            )
            .add_event::<MarkerDespawnEvent>()
            .init_gizmo_group::<MarkersGroup>()
            .init_gizmo_group::<GridViewGizmoGroup>();
    }
}

/// Add this bundle to a [`bevy::prelude::Entity`] with a [`ghx_grid::cartesian::grid::CartesianGrid`] if you are using a 3d camera `Camera3d`.
#[derive(Bundle)]
pub struct DebugGridView3dBundle {
    /// Debug view configuration of the grid
    pub view: DebugGridView,
    /// 3d-specific component-marker for the debug view
    pub view_type: DebugGridView3d,
}
impl Default for DebugGridView3dBundle {
    fn default() -> Self {
        Self {
            view_type: Default::default(),
            view: Default::default(),
        }
    }
}

/// Add this bundle to a [`bevy::prelude::Entity`] with a [`ghx_grid::cartesian::grid::CartesianGrid`] if you are using a 2d camera `Camera2d`.
#[derive(Bundle)]
pub struct DebugGridView2dBundle {
    /// Debug view configuration of the grid
    pub view: DebugGridView,
    /// 2d-specific component-marker for the debug view
    pub view_type: DebugGridView2d,
}
impl Default for DebugGridView2dBundle {
    fn default() -> Self {
        Self {
            view_type: Default::default(),
            view: Default::default(),
        }
    }
}

/// Returns a position as a [`Vec3`] in world units (center of the grid node) from a [`CartesianPosition`] accompanied by a `node_size`, the size of a grid node in world units.
#[inline]
pub fn get_translation_from_grid_pos_3d(grid_pos: &CartesianPosition, node_size: &Vec3) -> Vec3 {
    Vec3 {
        x: (grid_pos.x as f32 + 0.5) * node_size.x,
        y: (grid_pos.y as f32 + 0.5) * node_size.y,
        z: (grid_pos.z as f32 + 0.5) * node_size.z,
    }
}

/// Returns a position as a [`Vec3`] in world units (center of the grid node) from grid coordinates accompanied by a `node_size`, the size of a grid node in world units.
#[inline]
pub fn get_translation_from_grid_coords_3d(x: u32, y: u32, z: u32, node_size: &Vec3) -> Vec3 {
    Vec3 {
        x: (x as f32 + 0.5) * node_size.x,
        y: (y as f32 + 0.5) * node_size.y,
        z: (z as f32 + 0.5) * node_size.z,
    }
}

/// Returns a position as a [`Vec2`] in world units (center of the grid node) from a [`CartesianPosition`] accompanied by a `node_size`, the size of a grid node in world units.
#[inline]
pub fn get_translation_from_grid_pos_2d(grid_pos: &CartesianPosition, node_size: &Vec2) -> Vec2 {
    Vec2 {
        x: (grid_pos.x as f32 + 0.5) * node_size.x,
        y: (grid_pos.y as f32 + 0.5) * node_size.y,
    }
}

/// Returns a position as a [`Vec2`] in world units (center of the grid node) from grid coordinates accompanied by a `node_size`, the size of a grid node in world units.
#[inline]
pub fn get_translation_from_grid_coords_2d(x: u32, y: u32, node_size: &Vec2) -> Vec2 {
    Vec2 {
        x: (x as f32 + 0.5) * node_size.x,
        y: (y as f32 + 0.5) * node_size.y,
    }
}

/// Toggles the debug grids visibility
///
/// ### Example
///
/// Toggles On/Off debug grids by pressing F1
///
/// ```rust,ignore
///  app.add_systems(
///    Update,
///    toggle_debug_grids_visibilities.run_if(input_just_pressed(KeyCode::F1)),
///  );
/// ```
pub fn toggle_debug_grids_visibilities(mut grid_views: Query<&mut DebugGridView>) {
    for mut view in grid_views.iter_mut() {
        view.display_grid = !view.display_grid;
    }
}

/// Toggles the debug grids visibility
///
/// ### Example
///
/// Toggles On/Off debug grids by pressing F1
///
/// ```rust,ignore
///  app.add_systems(
///    Update,
///    toggle_grid_markers_visibilities.run_if(input_just_pressed(KeyCode::F1)),
///  );
/// ```
pub fn toggle_grid_markers_visibilities(mut grid_views: Query<&mut DebugGridView>) {
    for mut view in grid_views.iter_mut() {
        view.display_markers = !view.display_markers;
    }
}
