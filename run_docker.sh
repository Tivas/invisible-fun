# /bin/bash
docker stop eightt3
docker rm eightt3
docker pull tivasl/eightt3:latest
docker run -d --name eightt3 --restart unless-stopped -p 1032:1032 tivasl/eightt3