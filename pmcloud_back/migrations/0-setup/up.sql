CREATE TABLE books (
    id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(32) NOT NULL
);
CREATE TABLE pages (
    id INT PRIMARY KEY AUTO_INCREMENT,
    page_number INT NOT NULL,
    content TEXT NOT NULL,
    book_id INT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books (id)
);
