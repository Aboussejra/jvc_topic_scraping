pre-setup:
	docker run --name jvc_topics_db -v ~/jvc_topic_scrapping/my_sql_db:/var/lib/mysql -e MYSQL_ROOT_PASSWORD=lama -e MYSQL_DATABASE=jvc_topics_db -p 3306:3306 -it mysql:8.0
up:
	docker-compose -f docker-compose.yml up
debug_db:
	mysql --host=127.0.0.1 --port=3306 -u root -p
# Password is lama
