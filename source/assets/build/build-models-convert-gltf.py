#!/usr/bin/env python3
"""
Blender script to convert .blend files to glTF format.
Usage: blender --background --python build-models-convert-gltf.py -- <source.blend> <destination.gltf>
"""

import bpy
import sys

# Get source and destination from command line arguments (after the --)
source_file = sys.argv[-2]
destination_file = sys.argv[-1]

# Clear the default scene
bpy.ops.wm.read_factory_settings(use_empty=True)

# Import the blend file
bpy.ops.wm.open_mainfile(filepath=source_file)

# Export to glTF
bpy.ops.export_scene.gltf(
    filepath=destination_file,
    export_format='GLTF_SEPARATE',
    export_materials='EXPORT',
    export_normals=True,
    export_texcoords=True,
    export_cameras=False,
    export_lights=False,
)

print(f"Successfully exported {source_file} to {destination_file}")
