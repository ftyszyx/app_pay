# build

## build docker

```
docker compose build server
```

## test docker
```
如果有 bash：docker compose run --rm -it server bash
```

## run docker
```
docker compose up server 
```

## recreate docker
```
docker compose down server
docker compose up --force-recreate server
```

## run all
```
docker compose up -d
```

## build web
```
# python build_web.py --base-url https://appapi.bytefuse.cn/api
python build_web.py --base-url http://localhost:3000
```

cp -rf web /opt/1panel/www/sites/apps.bytefuse.cn/index


## rerun docker
docker compose -f docker-compose.release.yml up --force-recreate server