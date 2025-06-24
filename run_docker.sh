# /bin/bash
docker stop eightt3
docker rm eightt3
docker pull tivasl/eightt3:latest
docker run --name eightt3 -p 1032:1032 tivasl/eightt3