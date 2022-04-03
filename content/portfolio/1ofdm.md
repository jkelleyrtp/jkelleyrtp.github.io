+++
date = "2020-01-05T19:41:01+05:30"
image = "static/images/portfolio/ofdm.jpg"
showonlyimage = false
title = "WiFi from scratch: OFDM, Rust, and CUDA"
weight = 21
generatepage = false
link = "https://github.com/jkelleyrtp/ofdm"
+++

I implemented WiFi from the ground up on an Ettus B210 USRP. My final implementation was capable of 1 MBit transmission with sub-millisecond latency.

To implement WiFi, I used the Rust programming language to convert arbitrary data streams (bytes) into a MIMO OFDM packet. Encoding and decoding of the packets was accelerated with the ArrayFire library for CUDA-based GPGPU and deployed onto two NVIDIA Jetson TK2s.

The digital communications techniques I used included 64QAM for modulation, closed-loop frequency correction with the Shmidl-Cox algorithm, closed-loop phase offset correction, guardbands, colorspace compression, and live error correction with Hamming Codes.
