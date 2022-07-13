import mysql.connector
from mysql.connector import Error

try:
    connection = mysql.connector.connect(
        user="root",
        password="lama",
        host="localhost",
        port="3306",
        database="jvc_topic_scrapping_db",
    )
    if connection.is_connected():
        db_Info = connection.get_server_info()
        print("Connected to MySQL Server version ", db_Info)
        cursor = connection.cursor()
        cursor.execute("select database();")
        record = cursor.fetchone()
        print("You're connected to database: ", record)

except Error as e:
    print("Error while connecting to MySQL", e)
finally:
    if connection.is_connected():
        cursor.close()
        connection.close()
        print("MySQL connection is closed")
