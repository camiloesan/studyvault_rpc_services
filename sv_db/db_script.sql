drop database if exists study_vault;
create database study_vault default character set utf8mb4;
use study_vault;

create table users(
    user_id int not null auto_increment,
    user_type_id int not null,
    name varchar(32) not null,
    last_name varchar(64) not null,
    email varchar(64) not null,
    password varchar(64) not null,
    primary key(user_id),
    unique(user_id),
    unique(email)
);

create table user_types(
    user_type_id int not null auto_increment,
    user_type varchar(10) not null,
    primary key(user_type_id),
    unique(user_type_id)
);

create table subscriptions(
    user_id int not null,
    channel_id int not null,
    primary key(user_id, channel_id)
);

create table categories(
    category_id int not null auto_increment,
    name varchar(32) not null,
    primary key(category_id),
    unique(category_id)
);

create table channels(
    channel_id int not null auto_increment,
    creator_id int not null,
    name varchar(32) not null,
    description varchar(256),
    category_id int not null,
    primary key(channel_id),
    unique(channel_id)
);

create table posts(
    post_id int not null auto_increment,
    channel_id int not null,
    file_id varchar(36) not null,
    title varchar(32) not null,
    description varchar(256),
    publish_date date not null,
    primary key(post_id),
    unique(post_id),
    unique(file_id)
);

create table comments(
    comment_id int not null auto_increment,
    post_id int not null,
    user_id int not null,
    comment varchar(256) not null,
    publish_date date not null,
    primary key(comment_id),
    unique(comment_id)
);

create table files(
    file_id varchar(36) not null,
    name varchar(32) not null,
    primary key(file_id),
    unique(file_id)
);

-- foreign keys

alter table users
add constraint fk_users_user_types foreign key(user_type_id) references user_types(user_type_id) on delete cascade on update cascade;

alter table subscriptions
add constraint fk_subscriptions_users foreign key(user_id) references users(user_id) on delete cascade on update cascade,
add constraint fk_subscriptions_channels foreign key(channel_id) references channels(channel_id) on delete cascade on update cascade;

alter table channels
add constraint fk_channels_users foreign key(creator_id) references users(user_id) on delete cascade on update cascade,
add constraint fk_channels_categories foreign key(category_id) references categories(category_id) on delete cascade on update cascade;

alter table posts
add constraint fk_posts_channels foreign key(channel_id) references channels(channel_id) on delete cascade on update cascade,
add constraint fk_posts_files foreign key(file_id) references files(file_id) on delete cascade on update cascade;

alter table comments
add constraint fk_comments_post foreign key(post_id) references posts(post_id) on delete cascade on update cascade,
add constraint fk_comments_users foreign key(user_id) references users(user_id) on delete cascade on update cascade;

-- startup data

insert into user_types(user_type) values('Professor');
insert into user_types(user_type) values('Student');

insert into users(user_type_id, name, last_name, email, password) values(2, 'Camilo', 'Espejo Sánchez', 'zs21013861@estudiantes.uv.mx', '123456');
insert into users(user_type_id, name, last_name, email, password) values(1, 'Lizbeth', 'Rodríguez Mesa', 'lizrm@uv.mx', '123456');

-- users with hashed password
insert into users(user_type_id, name, last_name, email, password) values(1, 'Juan', 'Sánchez Meza', 'juan@uv.mx', 'ed08c290d7e22f7bb324b15cbadce35b0b348564fd2d5f95752388d86d71bcca'); -- password is juan
insert into users(user_type_id, name, last_name, email, password) values(2, 'Alejandra', 'Carabantes Martínez', 'zs21013862@estudiantes.uv.mx', '069fca009882e13e01c6b0559c9b14a4337c4495f83fd720965ec80f0770a699'); -- password is alejandra

insert into categories(name) values('Ingeniería de Requisitos');
insert into categories(name) values('Arquitectura de Software');
insert into categories(name) values('Paradigmas de Programación');
insert into categories(name) values('Estructuras de Datos');
insert into categories(name) values('Gestión de Proyectos');

insert into channels(creator_id, name, description, category_id) values(2, 'Ingeniería de Requisitos', 'Discusión sobre la recopilación y análisis de requisitos en proyectos de software.', 1);
insert into channels(creator_id, name, description, category_id) values(2, 'Arquitectura de Software', 'Canal dedicado a los patrones y estilos arquitectónicos en el desarrollo de software.', 2);
insert into channels(creator_id, name, description, category_id) values(2, 'Paradigmas de Programación', 'Explora los diferentes paradigmas de programación, desde el funcional hasta el orientado a objetos.', 3);
insert into channels(creator_id, name, description, category_id) values(2, 'Estructuras de Datos', 'Canal sobre estructuras de datos y su aplicación en la resolución de problemas algorítmicos.', 4);
insert into channels(creator_id, name, description, category_id) values(3, 'Gestión de Proyectos', 'Discusión sobre metodologías y herramientas para la gestión eficiente de proyectos de software.', 5);

insert into subscriptions(user_id, channel_id) values(1, 1);
insert into subscriptions(user_id, channel_id) values(1, 2);
insert into subscriptions(user_id, channel_id) values(4, 2);
