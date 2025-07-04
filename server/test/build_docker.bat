docker build -t test_server .
docker run -p 3000:3000 --env-file .env test_server