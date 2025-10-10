# build

## 一些命令

### build docker

```
docker compose build server
```

### test docker
```
如果有 bash：docker compose run --rm -it server bash
```

### run docker
```
docker compose up server 
```

### recreate docker
```
docker compose down server
docker compose up --force-recreate server
```

### run all
```
docker compose up -d
```



### rerun docker
docker compose -f docker-compose.release.yml up --force-recreate server


## 如何部署到服务器

### 构建服务器镜像(如果服务器有修改)

#### 方法1：git 创建一个tag并推送到github
会自动触发github actions发布到docker hub

#### 方法2：手动触发github actions

### 构建web(如果前端有修改)
```
# python build_web.py --base-url https://appapi.bytefuse.cn/api
python build_web.py --base-url http://localhost:3000
```

#### 提交git 

#### 在服务器上执行
```
update_web.sh
update_server.sh
```