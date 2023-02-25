#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

use crate::Snowflake;

/// Represents a Discord User Object.
/// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserObject {
    /// The user's id.
    id: Snowflake,

    /// The user's username, not unique across the platform.
    username: String,

    /// The user's 4-digit discord-tag.
    discriminator: String,

    /// the user's [avatar hash](https://discord.com/developers/docs/reference#image-formatting).
    avatar: Option<String>,

    /// Whether the user belongs to an OAuth2 application.
    #[serde(default)]
    bot: Option<bool>,

    /// Whether the user is an Official Discord System user (part of the urgent message system).
    #[serde(default)]
    system: Option<bool>,

    /// Whether the user has two factor enabled on their account.
    #[serde(default)]
    mfa_enabled: Option<bool>,

    /// The user's [banner hash](https://discord.com/developers/docs/reference#image-formatting).
    #[serde(default)]
    banner: Option<bool>,

    /// the user's banner color encoded as an integer representation of hexadecimal color code.
    #[serde(default)]
    accent_color: Option<u64>,

    /// The user's chosen [language option](https://discord.com/developers/docs/reference#locales).
    #[serde(default)]
    local: Option<String>,

    /// Whether the email on this account has been verified.
    #[serde(default)]
    verified: Option<bool>,

    /// The user's email.
    #[serde(default)]
    email: Option<String>,

    /// The [flags](https://discord.com/developers/docs/resources/user#user-object-user-flags). on a user's account.
    #[serde(default)]
    flags: Option<UserFlags>,

    /// The [type of Nitro subscription](https://discord.com/developers/docs/resources/user#user-object-premium-types). on a user's account.
    #[serde(default)]
    premium_type: Option<PremiumType>,

    /// The public flags on a user's account.
    #[serde(default)]
    public_flags: Option<UserFlags>,
}

bitflags! {
    /// Represents a Discord User Flags.
    /// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object-user-flags).
    #[derive(Serialize, Deserialize)]
    pub struct UserFlags: u64 {
        /// Discord Employee.
        const Staff = 1 << 0;

        /// Partnered Server Owner.
        const Partner = 1 << 1;

        /// HypeSquad Events Member.
        const Hypesquad = 1 << 2;

        /// Bug Hunter Level 1.
        const BugHunterLevel1 = 1 << 3;

        /// Bug Hunter Level 1.
        const HypeSquadOnlineHouse1 = 1 << 6;

        /// House Brilliance Member.
        const HypeSquadOnlineHouse2 = 1 << 7;

        /// House Balance Member.
        const HypeSquadOnlineHouse3 = 1 << 8;

        /// Early Nitro Supporter.
        const PremiumEarlySupporter = 1 << 9;

        /// User is a [team](https://discord.com/developers/docs/topics/teams).
        const TeamPseudoUser = 1 << 10;

        /// Bug Hunter Level 2.
        const BugHunterLevel2 = 1 << 14;

        /// Verified Bot.
        const VerifiedBot = 1 << 16;

        /// Early Verified Bot Developer.
        const VerifiedDeveloper = 1 << 17;

        /// Moderator Programs Alumni.
        const CertifiedModerator = 1 << 18;

        /// Bot uses only [HTTP interactions](https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction) and is shown in the online member list.
        const BotHTTPInteractions = 1 << 19;

        /// User is an [Active Developer](https://support-dev.discord.com/hc/articles/10113997751447).
        const ActiveDeveloper = 1 << 22;
    }
}

/// Represents a User Premium Type.
/// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object-premium-types).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    NitroBasic,
}

/// Represents a User Connection Object.
/// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object-premium-types).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionObject {
    /// Id of the connection account.
    id: String,

    /// The username of the connection account.
    name: String,

    /// The [service](https://discord.com/developers/docs/resources/user#connection-object-services) of the connection.
    #[serde(rename = "type")]
    ty: ConnectionService,

    /// Whether the connection is revoked.
    #[serde(default)]
    revoked: Option<bool>,

    // TODO: When write guild structure.
    /// An array of partial [server integration](https://discord.com/developers/docs/resources/guild#integration-object).
    #[serde(default)]
    integrations: Option<Value>,

    ///  Whether the connection is verified.
    verified: bool,

    /// Whether friend sync is enabled for this connection.
    friend_sync: bool,

    /// Whether friend sync is enabled for this connection.
    show_activity: bool,

    /// Whether this connection supports console voice transfer.
    two_way_link: bool,

    /// [Visibility](https://discord.com/developers/docs/resources/user#connection-object-visibility-types) of this connection.
    visibility: ConnectionVisibility,
}

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ConnectionService {
    BattleNet,
    EBay,
    EpicGames,
    Facebook,
    GitHub,
    Instagram,
    LeagueOfLegends,
    PayPal,
    PlayStationNetwork,
    Reddit,
    RiotGames,
    Spotify,
    Skype,
    Steam,
    TikTok,
    Twitch,
    Twitter,
    Xbox,
    YouTube,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionVisibility {
    /// Invisible to everyone except the user themselves.
    None,

    /// Visible to everyone
    Everyone,
}
