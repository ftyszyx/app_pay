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

## run all
```
docker compose up -d
```

## build web
```
python build_web.py --base-url https://appapi.bytefuse.cn/api
```