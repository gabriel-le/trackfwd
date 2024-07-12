use trackfwd::transform::{CoordsSystem, Transform};

static POSE: [[f32; 4]; 3] = [
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0, 1.0],
    [-1.0, 0.0, 0.0, 0.0],
];

#[test]
fn test_default_pose_matrix() {
    let coords_system = None;
    let transform = Transform::from_pose_matrix(&POSE, &coords_system);
    assert_eq!(transform.rotation.x, 0.0);
    assert_eq!(transform.rotation.y, 0.70710677);
    assert_eq!(transform.rotation.z, 0.0);
    assert_eq!(transform.rotation.w, 0.70710677);
    assert_eq!(transform.position.x, 0.0);
    assert_eq!(transform.position.y, 1.0);
    assert_eq!(transform.position.z, 0.0);
}

#[test]
fn test_unreal_pose_matrix() {
    let coords_system = Some(CoordsSystem::Unreal);
    let transform = Transform::from_pose_matrix(&POSE, &coords_system);
    assert_eq!(transform.rotation.x, 0.0);
    assert_eq!(transform.rotation.y, 0.0);
    assert_eq!(transform.rotation.z, -0.70710677);
    assert_eq!(transform.rotation.w, 0.70710677);
    assert_eq!(transform.position.x, 0.0);
    assert_eq!(transform.position.y, 0.0);
    assert_eq!(transform.position.z, 100.0);
}

#[test]
fn test_unity_pose_matrix() {
    let coords_system = Some(CoordsSystem::Unity);
    let transform = Transform::from_pose_matrix(&POSE, &coords_system);
    assert_eq!(transform.rotation.x, 0.0);
    assert_eq!(transform.rotation.y, 0.70710677);
    assert_eq!(transform.rotation.z, 0.0);
    assert_eq!(transform.rotation.w, 0.70710677);
    assert_eq!(transform.position.x, 0.0);
    assert_eq!(transform.position.y, 1.0);
    assert_eq!(transform.position.z, 0.0);
}

#[test]
fn test_blender_pose_matrix() {
    let coords_system = Some(CoordsSystem::Blender);
    let transform = Transform::from_pose_matrix(&POSE, &coords_system);
    assert_eq!(transform.rotation.x, 0.0);
    assert_eq!(transform.rotation.y, 0.70710677);
    assert_eq!(transform.rotation.z, 0.0);
    assert_eq!(transform.rotation.w, 0.70710677);
    assert_eq!(transform.position.x, 0.0);
    assert_eq!(transform.position.y, 1.0);
    assert_eq!(transform.position.z, 0.0);
}
