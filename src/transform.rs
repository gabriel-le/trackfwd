/// Quaternion struct
/// Holds rotation data
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
/// Vector3 struct
/// Holds position data
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Transform struct
/// Holds rotation and position data.
pub struct Transform {
    pub rotation: Quaternion,
    pub position: Vector3,
}

/// CoordsSystem enum
/// Presets for different coordinate systems. Can be used to convert the transform data from OpenVR to other systems.
#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum CoordsSystem {
    #[default]
    OpenVR, // Right-handed, Y-up. 1 unit = 1 meter
    Blender, // Right-handed, Z-up. 1 unit = 1 meter
    Unity,   // Left-handed, Y-up. 1 unit = 1 meter
    Unreal,  // Left-handed, Z-up. 1 unit = 1 centimeter
}

impl Quaternion {
    pub fn new() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new Quaternion from a pose matrix and a coordinate system.
    pub fn from_pose_matrix(pose: &[[f32; 4]; 3], coords_system: &Option<CoordsSystem>) -> Self {
        let w = (1.0 + pose[0][0] + pose[1][1] + pose[2][2]).max(0.0).sqrt() / 2.0;
        let x = (1.0 + pose[0][0] - pose[1][1] - pose[2][2]).max(0.0).sqrt() / 2.0;
        let y = (1.0 - pose[0][0] + pose[1][1] - pose[2][2]).max(0.0).sqrt() / 2.0;
        let z = (1.0 - pose[0][0] - pose[1][1] + pose[2][2]).max(0.0).sqrt() / 2.0;
        let x = x.copysign(pose[2][1] - pose[1][2]);
        let y = y.copysign(pose[0][2] - pose[2][0]);
        let z = z.copysign(pose[1][0] - pose[0][1]);

        match coords_system {
            Some(CoordsSystem::Unreal) => Quaternion {
                x,
                y: z,
                z: y * -1.0,
                w,
            },
            Some(CoordsSystem::Unity) => Quaternion {
                x: z,
                y: y,
                z: x,
                w: w * -1.0,
            },
            Some(CoordsSystem::Blender) => Quaternion {
                x: x,
                y: z,
                z: y,
                w: w,
            },
            _ => Quaternion { x, y, z, w },
        }
    }
}

impl Vector3 {
    pub fn new() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create a new Vector3 from a pose matrix and a coordinate system.
    pub fn from_pose_matrix(pose: &[[f32; 4]; 3], coords_system: &Option<CoordsSystem>) -> Self {
        match coords_system {
            Some(CoordsSystem::Unreal) => Vector3 {
                x: pose[2][3] * 100.0, // Z
                y: pose[0][3] * 100.0, // X
                z: pose[1][3] * 100.0, // Y
            },
            Some(CoordsSystem::Unity) => Vector3 {
                x: pose[2][3], // Z
                y: pose[1][3], // Y
                z: pose[0][3], // X
            },
            Some(CoordsSystem::Blender) => Vector3 {
                x: pose[0][3], // X
                y: pose[2][3], // Z
                z: pose[1][3], // Y
            },
            _ => Vector3 {
                x: pose[0][3], // X
                y: pose[1][3], // Y
                z: pose[2][3], // Z
            },
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            rotation: Quaternion::new(),
            position: Vector3::new(),
        }
    }

    /// Create a new Transform from a pose matrix and a coordinate system.
    pub fn from_pose_matrix(pose: &[[f32; 4]; 3], coords_system: &Option<CoordsSystem>) -> Self {
        Transform {
            rotation: Quaternion::from_pose_matrix(pose, coords_system),
            position: Vector3::from_pose_matrix(pose, coords_system),
        }
    }
}
