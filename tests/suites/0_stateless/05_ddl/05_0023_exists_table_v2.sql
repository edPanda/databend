SET enable_planner_v2=1;
DROP DATABASE IF EXISTS db_05_0023_v2;
CREATE DATABASE db_05_0023_v2;
USE db_05_0023_v2;

EXISTS TABLE t;
EXISTS TABLE db_05_0023_v2.t;
CREATE TABLE t(c1 int) ENGINE = Fuse;
EXISTS TABLE t;
EXISTS TABLE db_05_0023_v2.t;

DROP TABLE t ALL;

EXISTS TABLE db_05_0023_v2.t;
EXISTS TABLE t;

DROP database db_05_0023_v2;
