#!/bin/bash

cd /home/vishus/blogr-tls

cargo rustc --release -- --cfg production

# cp /home/vishus/blogr/target/release/blogr /home/vishus/blogr-exe
cp /home/vishus/blogr-tls/target/release/blogr /home/vishus/blogr-tls/blogr-tls-executable

# target/release/blogr > /home/vishus/blogr/logs/requests.log

# /home/vishus/blogr/target/release/blogr | tee -a /home/vishus/blogr/logs/requests.log


