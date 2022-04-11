ARG IMAGE=nginx:1.21
FROM ${IMAGE}

COPY dist/* /usr/share/nginx/html

