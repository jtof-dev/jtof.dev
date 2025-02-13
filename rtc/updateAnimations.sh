#!/bin/bash

rm -rf assets/animations

git clone --single-branch -b assets --depth 1 https://github.com/jtof-dev/ssbm-tech-animations.git assets/animations/

 rm -rf assets/animations/.git
