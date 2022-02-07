# Quick script that generates posts for the blog
# None of the posts have much detail, but are a nice a collage of projects/things

# quick quick list:
# - [] Hash
# - [] Quartz
# - [*] leaf systems
# - [*] fusion reactor
# - [] Conrad Challenge
# - [] Spatial Lab
# - [] Catalyst
# - [*] first robotics
# - [] unimolders
# - [] zero robotics
# - [] shop manager
# - [] barbour machine
# - [*] himcim

# # Projects for classes
# - [] Shadowboxing robot
# - [] Verilog CPU
# - [] Verilog GPU
# - [] BlackHoleAR
# - [] ThreadsApp
# - [] Discrete Map
# - [] Visualzing Schroedingers Equation
# - [] Audio Steganography


# lin
# pdes
# sigsys
# p&m
# DSA

topics = [
    ("Hash", "Engineering the next-generation business simulation platform", "hash"),
    ("Quartz", "Built cameras, sensors, and software for big construction", "quartz"),
    ("leaf systems", "Developing a real time positioning system for important assets", "leaf"),
    ("fusion reactor", "Built an inertial electrostatic confinement fusion reactor", "fusion"),
    ("Conrad Challenge", "Leading teams of alumni for the student entrepreneur's #1 resource", "conrad"),
    ("Spatial Lab", "Developing better algorithms for augmented reality", "spatial"),
    ("Catalyst", "Leading Olin's entrepreneurial community and startup incubator", "catalyst"),
    ("first robotics", "Founded and led a world-class FIRST Robotics Team (#5511)", "first"),
    ("unimolders", "Launched a zero-gravity injection molding system to the ISS", "unimolders"),
    ("Verilog GPU", "A GPU written in Verilog HDL with a hardware scheduler and compute cores", "veriloggpu"),
    ("ThreadsApp", "Search engine for dresses complete with principal component analysis and pose estimation", "threadsapp"),
    ("Beam Print", "Suite of Augmented Reality tools designed for engineering and manufacturing", "beam"),
    ("Visualzing Schroedingers Equation", "Solved and simulated the wave function for the hydrogen atom", "pdes"),
    ("Shadowboxing robot", "A kinect-powered 8-DoF robot that mimmics your every move", "shadowboxing"),
    ("Barbour Machine", "Worked as a manufacturing consultant for local product development firms", "barbour"),
    ("zero robotics", "Programmed floating robots on the ISS for MIT's Zero Robotics program", "zero"),
    ("himcim", "World finalist in the HiMCM mathematical modeling competition", "himcim"),
    ("Verilog CPU", "A simple MIPS standard CPU written in VerilogHDL", "verilogcpu"),
    ("BlackHoleAR", "Simulated and explore a blackhole using Apple's ARKit", "blackholear"),
    ("Discrete Map", "Learning map for major topics in discrete mathematics", "discrete"),
    ("Audio Steganography", "New ways of encoding secret messages into audio for fingerprinting and copyright", "audio"),
]

weight = 1
for post_name, post_text, file_name in topics:
    with open("genposts/"+file_name+".md", 'w') as file:
        a=(
'''+++
date = "2020-01-05T19:41:01+05:30"
image = "static/images/portfolio/{0}.jpg"
showonlyimage = false
title = "{1}"
weight = {2}
generatepage = false
+++

{3}
'''.format(file_name, post_name, weight, post_text)
        )
        file.write(a)
    weight = weight+1




