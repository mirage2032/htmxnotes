FROM mysql/mysql-server

ENV MYSQL_ROOT_PASSWORD=toor \
    MYSQL_USER=user \
    MYSQL_PASSWORD=pass \
    MYSQL_DATABASE=db \
    MYSQL_TZ=Europe/Bucharest

COPY ./init.sql /docker-entrypoint-initdb.d/init.sql

EXPOSE 3306

VOLUME ./data:/var/lib/mysql