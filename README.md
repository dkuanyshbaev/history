# Test

CREATE TABLE books (
id integer PRIMARY KEY AUTOINCREMENT ,
name text Not null,
description text Not null,
cover text Not null,
file text Not null);

    <!-- <form action="/books/delete/{{ book.id }}" method="post"> -->
    <!--     <input type="hidden" name="_method" value="delete" /> -->
    <!--     <button class="btn btn&#45;outline&#45;danger" type="submit" onclick="return confirm('Вы уверены, что хотите удалить это?');">Удалить</button> -->
    <!-- </form> -->
