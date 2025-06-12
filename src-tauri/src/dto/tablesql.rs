pub static BATCH_SQL: &str = r#"CREATE TABLE IF NOT EXISTS `batch` (
            `BatchID` int(11) NOT NULL AUTO_INCREMENT,
            `BatchTask` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
            `PotCode` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
            `GrillageCode` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
            `SampleCodeList` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
            `SampleSourceIDList` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
            `Operator` varchar(25) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
            `CreateTime` datetime DEFAULT NULL,
            `Status` varchar(15) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
            `UseTime` datetime DEFAULT NULL,
            `BoardType` int(11) DEFAULT NULL,
            `PotType` int(11) DEFAULT NULL,
            `Devno` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
            `Sign` varchar(15) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
            `OperationType` int(11) DEFAULT NULL,
            PRIMARY KEY (`BatchID`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#;

pub static USER_SQL: &str = r#"CREATE TABLE IF NOT EXISTS `users` (
            `id` INT AUTO_INCREMENT PRIMARY KEY,
            `username` VARCHAR(50) NOT NULL UNIQUE,
            `email` VARCHAR(100) NOT NULL,
            `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#;

pub static BOOK_SQL: &str = r#"CREATE TABLE `book` (
            `id` int(11) NOT NULL AUTO_INCREMENT,
            `price` float DEFAULT NULL,
            `sales` bigint(20) DEFAULT NULL,
            `publish_date` datetime DEFAULT NULL,
            `title` varchar(100) DEFAULT NULL,
            `author` varchar(100) DEFAULT NULL,
            `category` varchar(100) DEFAULT NULL,
            `rating` int(11) DEFAULT NULL,
            `img` varchar(100) DEFAULT NULL,
            `status` varchar(100) DEFAULT NULL,
            PRIMARY KEY (`id`)
            ) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4;"#;
