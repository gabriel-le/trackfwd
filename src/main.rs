use clap::Parser;
use std::{net::Ipv4Addr, panic, thread, time::Duration};

use openvr::{self as vr};
use osc::send_tracker_transform;
use transform::{CoordsSystem, Transform};

pub mod osc;
pub mod transform;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None, arg_required_else_help=true)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    target: Ipv4Addr,
    port: u16,
    #[arg(short, long)]
    coords: Option<CoordsSystem>,
}

fn main() {
    let ctx = match unsafe { vr::init(vr::ApplicationType::Other) } {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to initialize OpenVR: {:?}", err);
            return;
        }
    };
    let system = match ctx.system() {
        Ok(system) => system,
        Err(err) => {
            eprintln!("Failed to get system: {:?}", err);
            return;
        }
    };

    let args = Args::parse();
    let target = args.target;
    let port = args.port;
    let coords = args.coords;
    match &coords {
        Some(coords) => {
            println!("Using coordinate system: {:?}", coords);
        }
        None => {
            println!("Using default coordinate system: OpenVR");
        }
    }

    let poses = system
        .device_to_absolute_tracking_pose(vr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);

    let mut devices = Vec::new();
    for (index, pose) in poses.iter().enumerate() {
        if pose.device_is_connected() == false || pose.pose_is_valid() == false {
            continue;
        }
        match system.tracked_device_class(index as u32) {
            openvr::TrackedDeviceClass::GenericTracker => {
                match panic::catch_unwind(|| pose.device_to_absolute_tracking()) {
                    Ok(_) => {
                        devices.push(index as u32);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("Found {} trackers", devices.len());
    println!("Forwarding to {}:{}", target, port);
    println!("Press Ctrl+C to exit");
    loop {
        let poses = system
            .device_to_absolute_tracking_pose(vr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);

        for (index, device_index) in devices.iter().enumerate() {
            let pose = &poses[*device_index as usize];
            if pose.device_is_connected() == false || pose.pose_is_valid() == false {
                continue;
            }
            match panic::catch_unwind(|| pose.device_to_absolute_tracking()) {
                Ok(pose_matrix) => {
                    let transform = Transform::from_pose_matrix(pose_matrix, &coords);
                    send_tracker_transform(target, port, index as u32, &transform);
                }
                Err(err) => {
                    eprintln!("Failed to get pose matrix: {:?}", err);
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
