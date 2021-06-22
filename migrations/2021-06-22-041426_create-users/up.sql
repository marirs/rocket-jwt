CREATE TABLE users (
   username VARCHAR(255) NOT NULL,
   email VARCHAR(255) NOT NULL,
   password VARCHAR(255) NOT NULL,
   is_admin BOOLEAN NOT NULL DEFAULT false,
   token VARCHAR(255),
   PRIMARY KEY(username)
);
