//! Static avatar definitions and categories ported from `pstream-frontend/data/avatars.ts`.
//! Zero-cost compile-time static slices for WebAssembly runtime efficiency.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Avatar {
    pub name: &'static str,
    pub url: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AvatarCategory {
    pub id: &'static str,
    pub name: &'static str,
    pub avatars: &'static [Avatar],
}

pub static AVATAR_CATEGORIES: &[AvatarCategory] = &[
    AvatarCategory {
        id: "bridgerton",
        name: "Bridgerton",
        avatars: &[
            Avatar { name: "Anthony Bridgerton", url: "https://lh3.googleusercontent.com/d/1KzRtMnyHwlJYwjr09S3UjhLtEg51W-Lr" },
            Avatar { name: "Benedict Bridgerton", url: "https://lh3.googleusercontent.com/d/1YlWDXxAhiKlc6r641vEVqTqFsXobC__-" },
            Avatar { name: "Colin Bridgerton", url: "https://lh3.googleusercontent.com/d/1ay24h5pSUfQ339nf1yvYtuMVLsY-L_r5" },
            Avatar { name: "Daphne Basset", url: "https://lh3.googleusercontent.com/d/1iloFbK5eBE0JwAYsyEyZEYMahIaWLiZL" },
            Avatar { name: "Eloise Bridgerton", url: "https://lh3.googleusercontent.com/d/1eNNCBDTROwaoishIx9_fkx8YFucs12NE" },
            Avatar { name: "Francesca Bridgerton", url: "https://lh3.googleusercontent.com/d/1Ocb11OmSHJlRfzT6w97e_yuCp0B1X3EC" },
            Avatar { name: "Kathani Bridgerton", url: "https://lh3.googleusercontent.com/d/1QgeRATcBSnH8prA5S3Hqnxm3VwB7YB9G" },
            Avatar { name: "Lady Danbury", url: "https://lh3.googleusercontent.com/d/1qOo2XsRjVZsZQWUdLUJpHwuGEBRYlPQX" },
            Avatar { name: "Penelope Featherington", url: "https://lh3.googleusercontent.com/d/1Z__4y8VugluuVFwRdqG6eKaxwuy5rzc6" },
            Avatar { name: "Queen Charlotte", url: "https://lh3.googleusercontent.com/d/1Bvp9-Q7K6KqKxxxOz4l75ksVsJ4NhYmR" },
            Avatar { name: "Simon Basset", url: "https://lh3.googleusercontent.com/d/1bMUWIh4lqfpSrf5ifpdZ29GKC-gFCtJH" },
            Avatar { name: "Sophie Baek", url: "https://lh3.googleusercontent.com/d/1PVt6TyXP55YMM9c5tJNXev9Lsw1mDBlF" },
            Avatar { name: "Violet Bridgerton", url: "https://lh3.googleusercontent.com/d/1CIWh6dWbYek4Z7tLQVXkHEzBdN54xVFu" },
        ],
    },
    AvatarCategory {
        id: "one-piece",
        name: "One Piece",
        avatars: &[
            Avatar { name: "Monkey D. Luffy", url: "https://lh3.googleusercontent.com/d/1CCWWd9W3ODzxAn1lJ6TsKRYyAxdLxeq8" },
            Avatar { name: "Zoro", url: "https://lh3.googleusercontent.com/d/1HqM0dZKFN99eJw015CWCeZxpqix3EgT0" },
            Avatar { name: "Nami", url: "https://lh3.googleusercontent.com/d/1zqX1Q-0BIrG0taII5kqsL_zRNt1oVYE6" },
            Avatar { name: "Sanji", url: "https://lh3.googleusercontent.com/d/1ZL2FXFwrOjAiHuOXZ0SUu2gXZy3Jw3ts" },
            Avatar { name: "Chopper", url: "https://lh3.googleusercontent.com/d/1yZOQdzM_MPyWOpMgjqfd0S73KIoDAjLl" },
            Avatar { name: "Usopp", url: "https://lh3.googleusercontent.com/d/1T63TmoapNx9DLbTrOUWtu93XpqZkhQM5" },
            Avatar { name: "Arlong", url: "https://lh3.googleusercontent.com/d/1bUqGSDkhtgw1FtXZJqD5flQ-AMYjrBxa" },
            Avatar { name: "Alvida", url: "https://lh3.googleusercontent.com/d/1kRyzph--pyxtTGjEU35nt1GDPz70fMrU" },
            Avatar { name: "Shanks", url: "https://lh3.googleusercontent.com/d/1Sr0hpD0rTFJyyBbCWoKC0x6oD-rke-pU" },
            Avatar { name: "Going Merry", url: "https://lh3.googleusercontent.com/d/1my31WNqaaUo5v34oK08r_3o30Yvc6GAf" },
            Avatar { name: "Jolly Roger", url: "https://lh3.googleusercontent.com/d/18n1qSVxKIFQwX308WEsHC78-SlnSTRBu" },
        ],
    },
    AvatarCategory {
        id: "peaky-blinders",
        name: "Peaky Blinders",
        avatars: &[
            Avatar { name: "Tommy Shelby", url: "https://lh3.googleusercontent.com/d/1wW1ox6Uc1g368rqZ5CAphVSH84KW711n" },
            Avatar { name: "Duke", url: "https://lh3.googleusercontent.com/d/1lv3S6zkHP2cj7x1_z1Xas6cwN86bsrb-" },
            Avatar { name: "Hayden Stagg", url: "https://lh3.googleusercontent.com/d/1abCdQHYZ4A6thk6WvT0LMs7fgNVoXo1b" },
            Avatar { name: "John Beckett", url: "https://lh3.googleusercontent.com/d/1fMP16GVuLI_YE58zqgzokzrTrhCoN3cW" },
            Avatar { name: "Kaulo Chiriklo", url: "https://lh3.googleusercontent.com/d/1e1jV3_MfCKrv25cEy-y14HxVD3ChdJtT" },
        ],
    },
    AvatarCategory {
        id: "lucifer",
        name: "Lucifer",
        avatars: &[
            Avatar { name: "Lucifer Morningstar", url: "https://lh3.googleusercontent.com/d/1vuBOiPd9DknqbZpUXgMUEAUwZiGee52g" },
            Avatar { name: "Chloe Decker", url: "https://lh3.googleusercontent.com/d/1KtbqNziC8SxPDUJfMS2NGKhoaKaPUdos" },
            Avatar { name: "Mazikeen", url: "https://lh3.googleusercontent.com/d/1hygVLyg-7PsCkE1fdRByTje4OmgE7773" },
            Avatar { name: "Amenadiel", url: "https://lh3.googleusercontent.com/d/1dfWobx2-vWsArr2lQeZ_guWI2ZPvMQNo" },
        ],
    },
    AvatarCategory {
        id: "classics",
        name: "P-Stream Classics",
        avatars: &[
            Avatar { name: "Blue Fluffball", url: "https://lh3.googleusercontent.com/d/1i3UrprAcfhKSNaSwFE1FXwTD6NXOfjaV" },
            Avatar { name: "Gray Fluffball", url: "https://lh3.googleusercontent.com/d/1gKlTO0SLMJzk0RUqihW7n4ovugEZj9Jf" },
            Avatar { name: "Orange Fluffball", url: "https://lh3.googleusercontent.com/d/1I-MhzW-S8sQkJ72QOKQelNHnas41VWkn" },
            Avatar { name: "Bubblegum Princess", url: "https://lh3.googleusercontent.com/d/1ltTBpxXy_QxWDIJyyJHSYtwRemSb_fYb" },
            Avatar { name: "Green Alien", url: "https://lh3.googleusercontent.com/d/1_shb0mnchPaWk-F9anvInjBBRbGXJF7Z" },
            Avatar { name: "Panda Face", url: "https://lh3.googleusercontent.com/d/1MOcMHPqN0hFbkpoqjdirZl4jgI5VFVqo" },
            Avatar { name: "Red Anger", url: "https://lh3.googleusercontent.com/d/198aosLkzeCyglhaKy5vPMeWktSJhFui_" },
            Avatar { name: "Yellow Chicken", url: "https://lh3.googleusercontent.com/d/1ZYyoo8gUHeugXIa5ciA6pJySe3OPdkNB" },
        ],
    },
];

pub static ALL_AVATARS: &[&str] = &[
    "https://lh3.googleusercontent.com/d/1KzRtMnyHwlJYwjr09S3UjhLtEg51W-Lr",
    "https://lh3.googleusercontent.com/d/1YlWDXxAhiKlc6r641vEVqTqFsXobC__-",
    "https://lh3.googleusercontent.com/d/1ay24h5pSUfQ339nf1yvYtuMVLsY-L_r5",
    "https://lh3.googleusercontent.com/d/1iloFbK5eBE0JwAYsyEyZEYMahIaWLiZL",
    "https://lh3.googleusercontent.com/d/1eNNCBDTROwaoishIx9_fkx8YFucs12NE",
    "https://lh3.googleusercontent.com/d/1Ocb11OmSHJlRfzT6w97e_yuCp0B1X3EC",
    "https://lh3.googleusercontent.com/d/1QgeRATcBSnH8prA5S3Hqnxm3VwB7YB9G",
    "https://lh3.googleusercontent.com/d/1qOo2XsRjVZsZQWUdLUJpHwuGEBRYlPQX",
    "https://lh3.googleusercontent.com/d/1Z__4y8VugluuVFwRdqG6eKaxwuy5rzc6",
    "https://lh3.googleusercontent.com/d/1Bvp9-Q7K6KqKxxxOz4l75ksVsJ4NhYmR",
    "https://lh3.googleusercontent.com/d/1bMUWIh4lqfpSrf5ifpdZ29GKC-gFCtJH",
    "https://lh3.googleusercontent.com/d/1PVt6TyXP55YMM9c5tJNXev9Lsw1mDBlF",
    "https://lh3.googleusercontent.com/d/1CIWh6dWbYek4Z7tLQVXkHEzBdN54xVFu",
    "https://lh3.googleusercontent.com/d/1CCWWd9W3ODzxAn1lJ6TsKRYyAxdLxeq8",
    "https://lh3.googleusercontent.com/d/1HqM0dZKFN99eJw015CWCeZxpqix3EgT0",
    "https://lh3.googleusercontent.com/d/1zqX1Q-0BIrG0taII5kqsL_zRNt1oVYE6",
    "https://lh3.googleusercontent.com/d/1ZL2FXFwrOjAiHuOXZ0SUu2gXZy3Jw3ts",
    "https://lh3.googleusercontent.com/d/1yZOQdzM_MPyWOpMgjqfd0S73KIoDAjLl",
    "https://lh3.googleusercontent.com/d/1T63TmoapNx9DLbTrOUWtu93XpqZkhQM5",
    "https://lh3.googleusercontent.com/d/1bUqGSDkhtgw1FtXZJqD5flQ-AMYjrBxa",
    "https://lh3.googleusercontent.com/d/1kRyzph--pyxtTGjEU35nt1GDPz70fMrU",
    "https://lh3.googleusercontent.com/d/1Sr0hpD0rTFJyyBbCWoKC0x6oD-rke-pU",
    "https://lh3.googleusercontent.com/d/1my31WNqaaUo5v34oK08r_3o30Yvc6GAf",
    "https://lh3.googleusercontent.com/d/18n1qSVxKIFQwX308WEsHC78-SlnSTRBu",
    "https://lh3.googleusercontent.com/d/1wW1ox6Uc1g368rqZ5CAphVSH84KW711n",
    "https://lh3.googleusercontent.com/d/1lv3S6zkHP2cj7x1_z1Xas6cwN86bsrb-",
    "https://lh3.googleusercontent.com/d/1abCdQHYZ4A6thk6WvT0LMs7fgNVoXo1b",
    "https://lh3.googleusercontent.com/d/1fMP16GVuLI_YE58zqgzokzrTrhCoN3cW",
    "https://lh3.googleusercontent.com/d/1e1jV3_MfCKrv25cEy-y14HxVD3ChdJtT",
    "https://lh3.googleusercontent.com/d/1vuBOiPd9DknqbZpUXgMUEAUwZiGee52g",
    "https://lh3.googleusercontent.com/d/1KtbqNziC8SxPDUJfMS2NGKhoaKaPUdos",
    "https://lh3.googleusercontent.com/d/1hygVLyg-7PsCkE1fdRByTje4OmgE7773",
    "https://lh3.googleusercontent.com/d/1dfWobx2-vWsArr2lQeZ_guWI2ZPvMQNo",
    "https://lh3.googleusercontent.com/d/1i3UrprAcfhKSNaSwFE1FXwTD6NXOfjaV",
    "https://lh3.googleusercontent.com/d/1gKlTO0SLMJzk0RUqihW7n4ovugEZj9Jf",
    "https://lh3.googleusercontent.com/d/1I-MhzW-S8sQkJ72QOKQelNHnas41VWkn",
    "https://lh3.googleusercontent.com/d/1ltTBpxXy_QxWDIJyyJHSYtwRemSb_fYb",
    "https://lh3.googleusercontent.com/d/1_shb0mnchPaWk-F9anvInjBBRbGXJF7Z",
    "https://lh3.googleusercontent.com/d/1MOcMHPqN0hFbkpoqjdirZl4jgI5VFVqo",
    "https://lh3.googleusercontent.com/d/198aosLkzeCyglhaKy5vPMeWktSJhFui_",
    "https://lh3.googleusercontent.com/d/1ZYyoo8gUHeugXIa5ciA6pJySe3OPdkNB",
];

pub const DEFAULT_AVATAR: &str = ALL_AVATARS[0];

/// Finds an avatar by its URL across all categories.
pub fn get_avatar_by_url(url: &str) -> Option<&'static Avatar> {
    for cat in AVATAR_CATEGORIES {
        if let Some(av) = cat.avatars.iter().find(|a| a.url == url) {
            return Some(av);
        }
    }
    None
}

/// Finds an avatar category by its ID.
pub fn get_avatar_category(id: &str) -> Option<&'static AvatarCategory> {
    AVATAR_CATEGORIES.iter().find(|c| c.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_counts() {
        assert_eq!(AVATAR_CATEGORIES.len(), 5);
        assert_eq!(ALL_AVATARS.len(), 41);
        let total_nested: usize = AVATAR_CATEGORIES.iter().map(|c| c.avatars.len()).sum();
        assert_eq!(total_nested, 41);
    }

    #[test]
    fn test_avatar_urls_match() {
        let mut idx = 0;
        for cat in AVATAR_CATEGORIES {
            for av in cat.avatars {
                assert_eq!(av.url, ALL_AVATARS[idx]);
                idx += 1;
            }
        }
    }

    #[test]
    fn test_avatar_lookups() {
        let default_av = get_avatar_by_url(DEFAULT_AVATAR);
        assert!(default_av.is_some());
        assert_eq!(default_av.unwrap().name, "Anthony Bridgerton");

        assert!(get_avatar_by_url("https://invalid.url").is_none());

        let one_piece = get_avatar_category("one-piece");
        assert!(one_piece.is_some());
        assert_eq!(one_piece.unwrap().name, "One Piece");
        assert_eq!(one_piece.unwrap().avatars.len(), 11);
    }
}
