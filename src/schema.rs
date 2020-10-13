// This file is generated with `diesel print-schema` and edited with vim:
// %s:labs_\([^ ,]*\) :#[sql_name="labs_\1"]\r    \1:
// %s:labs_\([^ ,]*\),:\1,:

table! {
    #[sql_name="labs_cache"]
    cache(cache_key) {
        cache_key -> Varchar,
        created -> Integer,
        expire -> Integer,
        data -> Longblob,
    }
}

table! {
    #[sql_name="labs_contests"]
    contests(contest_id) {
        contest_id -> Integer,
        title -> Varchar,
        contest_type -> Varchar,
        start_time -> Bigint,
        options -> Integer,
        data -> Text,
        info -> Text,
        visible -> Bool,
        author_id -> Integer,
        allow_languages -> Varchar,
    }
}

table! {
    #[sql_name="labs_contest_problems"]
    contest_problems(contest_id, problem_id, user_id) {
        contest_id -> Integer,
        short_name -> Varchar,
        problem_id -> Integer,
        max_score -> Integer,
        is_with_code_review -> Tinyint,
        user_id -> Integer,
    }
}

table! {
    #[sql_name="labs_contest_users"]
    contest_users(contest_id, user_id) {
        contest_id -> Integer,
        user_id -> Integer,
        reg_status -> Integer,
        reg_data -> Tinytext,
    }
}

table! {
    #[sql_name="labs_groups"]
    groups(group_id) {
        group_id -> Integer,
        group_name -> Varchar,
        teacher_id -> Integer,
        group_description -> Nullable<Varchar>,
    }
}

table! {
    #[sql_name="labs_messages"]
    messages(message_id) {
        message_id -> Integer,
        from_user_id -> Integer,
        to_user_id -> Integer,
        in_reply_to -> Integer,
        message_state -> Integer,
        message_date -> Integer,
        message_subj -> Varchar,
        message_text -> Text,
    }
}

table! {
    #[sql_name="labs_problems"]
    problems(problem_id) {
        problem_id -> Unsigned<Integer>,
        title -> Varchar,
        description -> Text,
        attachment -> Varchar,
        complexity -> Integer,
        user_id -> Unsigned<Integer>,
        posted_time -> Integer,
        tex -> Nullable<Text>,
    }
}

table! {
    #[sql_name="labs_problems_ua"]
    problems_ua(problem_id) {
        problem_id -> Unsigned<Integer>,
        title -> Varchar,
        description -> Text,
        attachment -> Varchar,
        complexity -> Integer,
        user_id -> Unsigned<Integer>,
        posted_time -> Integer,
        tex -> Nullable<Text>,
    }
}

table! {
    #[sql_name="labs_sessions"]
    sessions(session_id) {
        session_id -> Char,
        user_agent -> Varchar,
        created_ip -> Unsigned<Integer>,
        updated_ip -> Unsigned<Integer>,
        created -> Integer,
        lifetime -> Integer,
        expire -> Integer,
        session_data -> Mediumtext,
    }
}

table! {
    #[sql_name="labs_solutions"]
    solutions(solution_id) {
        solution_id -> Unsigned<Integer>,
        problem_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        contest_id -> Nullable<Integer>,
        filename -> Varchar,
        checksum -> Varchar,
        lang_id -> Unsigned<Integer>,
        check_type -> Char,
        posted_time -> Integer,
        checked_time -> Unsigned<Integer>,
        contest_time -> Unsigned<Integer>,
        test_result -> Integer,
        test_score -> Unsigned<Decimal>,
        score -> Decimal,
        module_val -> Integer,
        compile_error -> Nullable<Text>,
        is_passed -> Tinyint,
    }
}

table! {
    #[sql_name="labs_tests"]
    tests(test_id) {
        test_id -> Unsigned<Integer>,
        solution_id -> Unsigned<Integer>,
        test_no -> Unsigned<Integer>,
        test_result -> Unsigned<Integer>,
        test_score -> Unsigned<Decimal>,
        test_time -> Unsigned<Integer>,
        test_mem -> Unsigned<Integer>,
    }
}

table! {
    #[sql_name="labs_users"]
    users(user_id) {
        user_id -> Unsigned<Integer>,
        email -> Varchar,
        password -> Varchar,
        nickname -> Varchar,
        birthday -> Date,
        access -> Unsigned<Integer>,
        created -> Integer,
        lastlogin -> Integer,
        options -> Unsigned<Integer>,
        messages -> Integer,
        avatar -> Varchar,
        city_name -> Varchar,
        region_name -> Varchar,
        country_name -> Varchar,
        FIO -> Varchar,
        job -> Varchar,
        is_activated -> Tinyint,
    }
}

table! {
    #[sql_name="labs_user_group_relationships"]
    user_group_relationships(user_id, group_id) {
        user_id -> Integer,
        group_id -> Integer,
    }
}

table! {
    #[sql_name="labs_user_teacher_relationships"]
    user_teacher_relationships(user_id, teacher_id) {
        user_id -> Integer,
        teacher_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    cache,
    contests,
    contest_problems,
    contest_users,
    groups,
    messages,
    problems,
    problems_ua,
    sessions,
    solutions,
    tests,
    users,
    user_group_relationships,
    user_teacher_relationships,
);
