#![allow(non_snake_case)]

pub mod ban;
pub mod clear;
pub mod deafen;
pub mod getBanReason;
pub mod isBanned;
pub mod isTimedOut;
pub mod kick;
pub mod mute;
pub mod timeout;
pub mod unban;
pub mod undeafen;
pub mod unmute;
pub mod untimeOut;

// Advanced moderation functions
pub mod spamDetect;
pub mod raidDetect;
pub mod duplicateDetect;
pub mod mentionSpamDetect;
pub mod linkSpamDetect;
pub mod capsDetect;
pub mod emojiSpamDetect;
pub mod newAccountDetect;
