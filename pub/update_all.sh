git pull --rebase
cp -rf web/* /opt/1panel/www/sites/apps.bytefuse.cn/index/*
docker compose -f docker-compose.release.yml down server
docker compose -f docker-compose.release.yml up -d --force-recreate server