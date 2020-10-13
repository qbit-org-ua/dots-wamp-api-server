CREATE TABLE `labs_cache` (
    `cache_key` varchar(64) NOT NULL,
    `created` int(11) NOT NULL,
    `expire` int(11) NOT NULL,
    `data` longblob NOT NULL,
    PRIMARY KEY (`cache_key`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_contests` (
    `contest_id` int(11) NOT NULL AUTO_INCREMENT,
    `title` varchar(255) NOT NULL,
    `contest_type` varchar(30) NOT NULL,
    `start_time` bigint(20) NOT NULL DEFAULT '0',
    `options` int(11) NOT NULL,
    `data` text NOT NULL,
    `info` text NOT NULL,
    `visible` tinyint(1) NOT NULL DEFAULT '0',
    `author_id` int(11) NOT NULL DEFAULT '0',
    `allow_languages` varchar(255) NOT NULL DEFAULT '2 4 3 39',
    PRIMARY KEY (`contest_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_contest_problems` (
    `contest_id` int(11) NOT NULL,
    `short_name` varchar(20) NOT NULL,
    `problem_id` int(11) NOT NULL,
    `max_score` int(11) NOT NULL DEFAULT '100',
    `is_with_code_review` tinyint(4) NOT NULL DEFAULT '0',
    `user_id` int(11) NOT NULL DEFAULT '0',
    PRIMARY KEY (`contest_id`, `problem_id`, `user_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_contest_users` (
    `contest_id` int(11) NOT NULL,
    `user_id` int(11) NOT NULL,
    `reg_status` int(11) NOT NULL,
    `reg_data` tinytext NOT NULL,
    PRIMARY KEY (`contest_id`, `user_id`),
    KEY `user_id` (`user_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_groups` (
    `group_id` int(11) NOT NULL AUTO_INCREMENT,
    `group_name` varchar(50) NOT NULL DEFAULT '0',
    `teacher_id` int(11) NOT NULL DEFAULT '0',
    `group_description` varchar(5000) DEFAULT NULL,
    PRIMARY KEY (`group_id`),
    UNIQUE KEY `group_id` (`group_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_messages` (
    `message_id` int(11) NOT NULL AUTO_INCREMENT,
    `from_user_id` int(11) NOT NULL,
    `to_user_id` int(11) NOT NULL,
    `in_reply_to` int(11) NOT NULL DEFAULT '0',
    `message_state` int(11) NOT NULL DEFAULT '0',
    `message_date` int(11) NOT NULL DEFAULT '0',
    `message_subj` varchar(250) NOT NULL,
    `message_text` text NOT NULL,
    PRIMARY KEY (`message_id`),
    KEY `to_user_id` (`to_user_id`, `message_state`),
    KEY `from_user_id` (`from_user_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_problems` (
    `problem_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `title` varchar(250) NOT NULL DEFAULT '',
    `description` text NOT NULL,
    `attachment` varchar(60) NOT NULL DEFAULT '',
    `complexity` int(11) NOT NULL DEFAULT '0',
    `user_id` int(10) unsigned NOT NULL DEFAULT '0',
    `posted_time` int(11) NOT NULL DEFAULT '0',
    `tex` text,
    PRIMARY KEY (`problem_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 PACK_KEYS = 0;
CREATE TABLE `labs_problems_ua` (
    `problem_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `title` varchar(250) NOT NULL DEFAULT '',
    `description` text NOT NULL,
    `attachment` varchar(60) NOT NULL DEFAULT '',
    `complexity` int(11) NOT NULL DEFAULT '0',
    `user_id` int(10) unsigned NOT NULL DEFAULT '0',
    `posted_time` int(11) NOT NULL DEFAULT '0',
    `tex` text,
    PRIMARY KEY (`problem_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 PACK_KEYS = 0;
CREATE TABLE `labs_sessions` (
    `session_id` char(16) COLLATE latin1_bin NOT NULL,
    `user_agent` varchar(80) COLLATE latin1_bin NOT NULL,
    `created_ip` int(12) unsigned NOT NULL,
    `updated_ip` int(12) unsigned NOT NULL,
    `created` int(11) NOT NULL,
    `lifetime` int(11) NOT NULL,
    `expire` int(11) NOT NULL,
    `session_data` mediumtext COLLATE latin1_bin NOT NULL,
    PRIMARY KEY (`session_id`),
    KEY `expire` (`expire`)
) ENGINE = InnoDB DEFAULT CHARSET = latin1 COLLATE = latin1_bin;
CREATE TABLE `labs_solutions` (
    `solution_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `problem_id` int(10) unsigned NOT NULL DEFAULT '0',
    `user_id` int(10) unsigned NOT NULL DEFAULT '0',
    `contest_id` int(11) DEFAULT NULL,
    `filename` varchar(120) NOT NULL DEFAULT '',
    `checksum` varchar(80) NOT NULL DEFAULT '',
    `lang_id` int(10) unsigned NOT NULL DEFAULT '0',
    `check_type` char(1) NOT NULL DEFAULT 'F',
    `posted_time` int(10) NOT NULL DEFAULT '0',
    `checked_time` int(10) unsigned NOT NULL DEFAULT '0',
    `contest_time` int(10) unsigned NOT NULL DEFAULT '0',
    `test_result` int(10) NOT NULL DEFAULT '0',
    `test_score` decimal(10, 2) unsigned NOT NULL DEFAULT '0.00',
    `score` decimal(10, 2) NOT NULL DEFAULT '0.00',
    `module_val` int(11) NOT NULL DEFAULT '0',
    `compile_error` text,
    `is_passed` tinyint(4) NOT NULL DEFAULT '1',
    PRIMARY KEY (`solution_id`),
    KEY `test_result` (`test_result`),
    KEY `user_id` (`contest_id`, `user_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 PACK_KEYS = 0;
CREATE TABLE `labs_tests` (
    `test_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `solution_id` int(10) unsigned NOT NULL DEFAULT '0',
    `test_no` int(10) unsigned NOT NULL DEFAULT '0',
    `test_result` int(10) unsigned NOT NULL DEFAULT '0',
    `test_score` decimal(10, 2) unsigned NOT NULL DEFAULT '0.00',
    `test_time` int(10) unsigned NOT NULL,
    `test_mem` int(10) unsigned NOT NULL,
    PRIMARY KEY (`test_id`),
    UNIQUE KEY `solution_test` (`solution_id`, `test_no`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 PACK_KEYS = 0;
CREATE TABLE `labs_users` (
    `user_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `email` varchar(120) NOT NULL,
    `password` varchar(64) NOT NULL DEFAULT '',
    `nickname` varchar(64) NOT NULL,
    `birthday` date NOT NULL,
    `access` int(10) unsigned NOT NULL DEFAULT '0',
    `created` int(11) NOT NULL DEFAULT '0',
    `lastlogin` int(11) NOT NULL DEFAULT '0',
    `options` int(10) unsigned NOT NULL,
    `messages` int(11) NOT NULL DEFAULT '0',
    `avatar` varchar(80) NOT NULL,
    `city_name` varchar(100) NOT NULL DEFAULT '',
    `region_name` varchar(100) NOT NULL DEFAULT '',
    `country_name` varchar(100) NOT NULL DEFAULT '',
    `FIO` varchar(200) NOT NULL DEFAULT '',
    `job` varchar(200) NOT NULL DEFAULT '',
    `is_activated` tinyint(4) NOT NULL DEFAULT '0',
    PRIMARY KEY (`user_id`),
    UNIQUE KEY `email` (`email`),
    UNIQUE KEY `nickname` (`nickname`),
    KEY `options` (`options`),
    KEY `lastlogin` (`lastlogin`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 PACK_KEYS = 0;
CREATE TABLE `labs_user_group_relationships` (
    `user_id` int(11) DEFAULT NULL,
    `group_id` int(11) DEFAULT NULL,
    PRIMARY KEY `user_id_group_id` (`user_id`, `group_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
CREATE TABLE `labs_user_teacher_relationships` (
    `user_id` int(11) DEFAULT NULL,
    `teacher_id` int(11) DEFAULT NULL,
    PRIMARY KEY `user_id_teacher_id` (`user_id`, `teacher_id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
