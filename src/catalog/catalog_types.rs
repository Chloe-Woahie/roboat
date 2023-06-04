use super::request_types;
#[allow(unused_imports)]
use crate::{Client, RoboatError};
use serde::{Deserialize, Serialize};

const AVATAR_CATALOG_SEARCH_BASE_URL: &str = "https://catalog.roblox.com/v1/search/items?";

/// An enum representing the overall high level type of the item (Asset or Bundle)
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemType {
    /// An individual asset.
    #[default]
    Asset,
    /// A bundle (such as an animation package).
    Bundle,
}

/// An enum representing the type of the asset (hat, shirt, gear).
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum AssetType {
    #[default]
    TShirt,
    Hat,
    Shirt,
    Pants,
    Head,
    Face,
    Gear,
    Arms,
    Legs,
    Torso,
    RightArm,
    LeftArm,
    LeftLeg,
    RightLeg,
    HairAccessory,
    FaceAccessory,
    NeckAccessory,
    ShoulderAccessory,
    FrontAccessory,
    BackAccessory,
    WaistAccessory,
    ClimbAnimation,
    DeathAnimation,
    FallAnimation,
    IdleAnimation,
    JumpAnimation,
    RunAnimation,
    SwimAnimation,
    WalkAnimation,
    PoseAnimation,
    EmoteAnimation,
}

/// An enum representing the type of bundle (BodyParts or AvatarAnimations).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum BundleType {
    #[default]
    BodyParts,
    AvatarAnimations,
}

/// An enum representing the genre of an item (war, funny). Only used when returning
/// info from item_details.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum Genre {
    #[default]
    All,
    Tutorial,
    Scary,
    TownAndCity,
    War,
    Funny,
    Fantasy,
    Adventure,
    SciFi,
    Pirate,
    FPS,
    RPG,
    Sports,
    Ninja,
    WildWest,
}

/// The status of an item (Sale, Exclusive).
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemStatus {
    #[default]
    New,
    Sale,
    XboxExclusive,
    AmazonExclusive,
    GooglePlayExclusive,
    IosExclusive,
    SaleTimer,
}

/// Restriction on an item (ThirteenPlus, Limited). Not sure why limited
/// is here but I guess that's how they do it.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum ItemRestriction {
    #[default]
    ThirteenPlus,
    LimitedUnique,
    Limited,
    Rthro,
    /// Appears to be used only for "new" limiteds (including ugc limiteds).
    Collectible,
}

/// The price status of an item. Only applies to items not on sale (Free, Offsale).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum PriceStatus {
    #[default]
    Free,
    #[serde(alias = "Off Sale")]
    Offsale,
    #[serde(alias = "No Resellers")]
    NoResellers,
}

/// The broad category of an item for use in search (Clothing, Collectables).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum Category {
    #[default]
    Featured,
    All,
    Collectibles,
    Clothing,
    BodyParts,
    Gear,
    Accessories,
    AvatarAnimations,
    CommunityCreations,
}

impl Category {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Featured => 0,
            Self::All => 1,
            Self::Collectibles => 2,
            Self::Clothing => 3,
            Self::BodyParts => 4,
            Self::Gear => 5,
            Self::Accessories => 11,
            Self::AvatarAnimations => 12,
            Self::CommunityCreations => 13,
        }
    }
}

/// A time period for when a sort applies.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum SortAggregation {
    #[default]
    PastDay,
    PastWeek,
    PastMonth,
    AllTime,
}

impl SortAggregation {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::PastDay => 0,
            Self::PastWeek => 1,
            Self::PastMonth => 2,
            Self::AllTime => 3,
        }
    }
}

/// Sorting types that can be used in an item search.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum SortType {
    #[default]
    Relevance,
    Favorited,
    Sales,
    Updated,
    PriceAsc,
    PriceDesc,
}

impl SortType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Relevance => 0,
            Self::Favorited => 1,
            Self::Sales => 2,
            Self::Updated => 3,
            Self::PriceAsc => 4,
            Self::PriceDesc => 5,
        }
    }
}

/// Sort between different sale types of assets, used when searching.
/// Values can be from here <https://create.roblox.com/docs/reference/engine/enums/SalesTypeFilter>.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum SalesTypeFilter {
    #[default]
    All,
    Collectibles,
    Premium,
}

impl SalesTypeFilter {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::All => 1,
            Self::Collectibles => 2,
            Self::Premium => 3,
        }
    }
}

/// Limit the amount of results shown by the API when searching.
/// Values can be found when entering an invalid limit to the API <https://catalog.roblox.com/v1/search/items?limit=110>.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum Limit {
    #[default]
    Ten,
    TwentyEight,
    Thirty,
    Fifty,
    Sixty,
    Hundred,
    HundredTwenty,
}

impl Limit {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Limit::Ten => 10,
            Limit::TwentyEight => 28,
            Limit::Thirty => 30,
            Limit::Fifty => 50,
            Limit::Sixty => 60,
            Limit::Hundred => 100,
            Limit::HundredTwenty => 120,
        }
    }
}

/// A subcategory for items, used when searching.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum Subcategory {
    Featured,
    #[default]
    All,
    Collectibles,
    Clothing,
    BodyParts,
    Gear,
    Hats,
    Faces,
    Shirts,
    TShirts,
    Pants,
    Heads,
    Accessories,
    HairAccessories,
    FaceAccessories,
    NeckAccessories,
    ShoulderAccessories,
    FrontAccessories,
    BackAccessories,
    WaistAccessories,
    AvatarAnimations,
    Bundles,
    AnimationBundles,
    EmoteAnimations,
    CommunityCreations,
    Melee,
    Ranged,
    Explosive,
    PowerUp,
    Navigation,
    Musical,
    Social,
    Building,
    Transport,
}

impl Subcategory {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::Featured => 0,
            Self::All => 1,
            Self::Collectibles => 2,
            Self::Clothing => 3,
            Self::BodyParts => 4,
            Self::Gear => 5,
            Self::Hats => 9,
            Self::Faces => 10,
            Self::Shirts => 12,
            Self::TShirts => 13,
            Self::Pants => 14,
            Self::Heads => 15,
            Self::Accessories => 19,
            Self::HairAccessories => 20,
            Self::FaceAccessories => 21,
            Self::NeckAccessories => 22,
            Self::ShoulderAccessories => 23,
            Self::FrontAccessories => 24,
            Self::BackAccessories => 25,
            Self::WaistAccessories => 26,
            Self::AvatarAnimations => 27,
            Self::Bundles => 37,
            Self::AnimationBundles => 38,
            Self::EmoteAnimations => 39,
            Self::CommunityCreations => 40,
            Self::Melee => 41,
            Self::Ranged => 42,
            Self::Explosive => 43,
            Self::PowerUp => 44,
            Self::Navigation => 45,
            Self::Musical => 46,
            Self::Social => 47,
            Self::Building => 48,
            Self::Transport => 49,
        }
    }
}

/// Additional details for premium pricing.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct PremiumPricing {
    /// The discount percentage in the form of a value from 0-100.
    #[serde(alias = "premiumDiscountPercentage")]
    pub premium_discount_percentage: u64,
    /// The price of the item for premium users.
    #[serde(alias = "premiumPriceInRobux")]
    pub premium_price_in_robux: u64,
}

/// A struct containing (mostly) all the fields possibly returned from <https://catalog.roblox.com/v1/catalog/items/details>.
///
/// Returned from [`Client::item_details`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct ItemDetails {
    /// Either the asset id, or the bundle id, depending on the [`Self::item_type`].
    pub id: u64,
    /// The type of item (Asset or Bundle).
    pub item_type: ItemType,
    /// Exclusive with [`ItemDetails::bundle_type`].
    pub asset_type: Option<AssetType>,
    /// Exclusive with [`ItemDetails::asset_type`].
    pub bundle_type: Option<BundleType>,
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// The product id of the item. This is different from the asset/bundle id.
    /// This is most notably used when buying limiteds. Is not used for "new" limiteds.
    pub product_id: Option<u64>,
    /// Only exists if the [`ItemDetails::item_type`] is a [`ItemType::Asset`].
    pub genres: Option<Vec<Genre>>,
    /// The statuses of an item (e.g., New, Sale). Does not exist on "new" limiteds.
    pub item_statuses: Option<Vec<ItemStatus>>,
    /// The restrictions on an item (e.g., ThirteenPlus, Limited).
    ///
    /// If there are none and the item is a non-tradable limited, then the field does not exist.
    /// Otherwise, the vector exists but the length is zero.
    pub item_restrictions: Option<Vec<ItemRestriction>>,
    /// Whether the creator is verified by Roblox.
    pub creator_has_verified_badge: bool,
    /// The type of creator that created the item (User or Group).
    pub creator_type: CreatorType,
    /// The id (group or user) of the creator. The value is 1 if the creator is Roblox.
    pub creator_id: u64,
    /// The name of the creator. The value is "Roblox" if the creator is Roblox.
    pub creator_name: String,
    /// Coincides with price if the item is a non-limited,
    /// and lowest price if item is a limited.
    ///
    /// If the item is offsale, the price is 0.
    /// However, if the price is a limited and no resellers exist,
    /// then the price does not exist.
    pub price: Option<u64>,
    /// For some reason, if details for multiple items are requested from
    /// the item details endpoint, this field is not present.
    pub favorite_count: Option<u64>,
    /// Only exists if the item has a special price status.
    pub price_status: Option<PriceStatus>,
    /// Only exists if the item has special premium pricing.
    pub premium_pricing: Option<PremiumPricing>,
    /// The remaining stock of an item. Only applies to "new" limiteds.
    pub remaining_stock: Option<u64>,
    /// The total stock of an item. Only applies to "new" limiteds.
    pub total_stock: Option<u64>,
    /// The id needed to purchase a "new" limited. This replaces the
    /// product id. Although this is an id, this is a String instead of a u64.
    pub collectible_item_id: Option<String>,
}

/// Contains an item id and its type. Used as part of a parameter in [`Client::item_details`], and used as
/// part of a response in [`Client::avatar_catalog_search`].
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub struct Item {
    /// The type of the item (Asset or Bundle).
    #[serde(alias = "itemType")]
    pub item_type: ItemType,
    /// The id of the item, or of the bundle.
    /// In the [`Client::item_details`] endpoint, it acts as both, depending on the [`Self::item_type`].
    pub id: u64,
}

impl TryFrom<u64> for AssetType {
    type Error = RoboatError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(AssetType::TShirt),
            8 => Ok(AssetType::Hat),
            11 => Ok(AssetType::Shirt),
            12 => Ok(AssetType::Pants),
            17 => Ok(AssetType::Head),
            18 => Ok(AssetType::Face),
            19 => Ok(AssetType::Gear),
            25 => Ok(AssetType::Arms),
            26 => Ok(AssetType::Legs),
            27 => Ok(AssetType::Torso),
            28 => Ok(AssetType::RightArm),
            29 => Ok(AssetType::LeftArm),
            30 => Ok(AssetType::LeftLeg),
            31 => Ok(AssetType::RightLeg),
            41 => Ok(AssetType::HairAccessory),
            42 => Ok(AssetType::FaceAccessory),
            43 => Ok(AssetType::NeckAccessory),
            44 => Ok(AssetType::ShoulderAccessory),
            45 => Ok(AssetType::FrontAccessory),
            46 => Ok(AssetType::BackAccessory),
            47 => Ok(AssetType::WaistAccessory),
            48 => Ok(AssetType::ClimbAnimation),
            49 => Ok(AssetType::DeathAnimation),
            50 => Ok(AssetType::FallAnimation),
            51 => Ok(AssetType::IdleAnimation),
            52 => Ok(AssetType::JumpAnimation),
            53 => Ok(AssetType::RunAnimation),
            54 => Ok(AssetType::SwimAnimation),
            55 => Ok(AssetType::WalkAnimation),
            56 => Ok(AssetType::PoseAnimation),
            61 => Ok(AssetType::EmoteAnimation),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

impl TryFrom<u64> for BundleType {
    type Error = RoboatError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BundleType::BodyParts),
            2 => Ok(BundleType::AvatarAnimations),
            _ => Err(RoboatError::MalformedResponse),
        }
    }
}

impl TryFrom<request_types::ItemDetailsRaw> for ItemDetails {
    type Error = RoboatError;

    fn try_from(value: request_types::ItemDetailsRaw) -> Result<Self, Self::Error> {
        let asset_type = match value.asset_type {
            Some(asset_type_id) => {
                let asset_type = AssetType::try_from(asset_type_id)?;
                Some(asset_type)
            }
            None => None,
        };

        let bundle_type = match value.bundle_type {
            Some(bundle_type_id) => {
                let bundle_type = BundleType::try_from(bundle_type_id)?;
                Some(bundle_type)
            }
            None => None,
        };

        let id = value.id.ok_or(RoboatError::MalformedResponse)?;
        let item_type = value.item_type.ok_or(RoboatError::MalformedResponse)?;
        let name = value.name.ok_or(RoboatError::MalformedResponse)?;
        let description = value.description.ok_or(RoboatError::MalformedResponse)?;
        let product_id = value.product_id;
        let creator_type = value.creator_type.ok_or(RoboatError::MalformedResponse)?;
        let item_statuses = value.item_status;

        let item_restrictions = value.item_restrictions;

        let creator_has_verified_badge = value
            .creator_has_verified_badge
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_id = value
            .creator_target_id
            .ok_or(RoboatError::MalformedResponse)?;

        let creator_name = value
            .creator_name
            .clone()
            .ok_or(RoboatError::MalformedResponse)?;

        let genres = value.genres;
        let favorite_count = value.favorite_count;
        let price_status = value.price_status;
        let premium_pricing = value.premium_pricing;

        // If the price is None, use the lowest price (used for limiteds).
        // If neither exists, the item has no resellers and the price
        // does not exist.
        let price = match value.price {
            Some(x) => Some(x),
            None => value.lowest_price,
        };

        let remaining_stock = value.units_available_for_consumption;
        let total_stock = value.total_quantity;
        let collectible_item_id = value.collectible_item_id;

        Ok(Self {
            id,
            item_type,
            asset_type,
            bundle_type,
            name,
            description,
            product_id,
            genres,
            item_statuses,
            item_restrictions,
            creator_has_verified_badge,
            creator_type,
            creator_id,
            creator_name,
            price,
            favorite_count,
            price_status,
            premium_pricing,
            remaining_stock,
            total_stock,
            collectible_item_id,
        })
    }
}

/// The type of a creator (User, Group).
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
#[allow(missing_docs)]
pub enum CreatorType {
    #[default]
    User,
    Group,
}

impl CreatorType {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::User => 1,
            Self::Group => 2,
        }
    }
}

/// The allowed limits in a catalog search query.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub(super) enum QueryLimit {
    #[default]
    Ten,
    TwentyEight,
    Thirty,
}

impl QueryLimit {
    pub(super) fn as_u8(&self) -> u8 {
        match self {
            Self::Ten => 10,
            Self::TwentyEight => 28,
            Self::Thirty => 30,
        }
    }
}

/// These are only used when making a search query.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum QueryGenre {
    #[default]
    TownAndCity,
    Medieval,
    SciFi,
    Fighting,
    Horror,
    Naval,
    Adventure,
    Sports,
    Comedy,
    Western,
    Military,
    Building,
    FPS,
    RPG,
}

impl QueryGenre {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Self::TownAndCity => 1,
            Self::Medieval => 2,
            Self::SciFi => 3,
            Self::Fighting => 4,
            Self::Horror => 5,
            Self::Naval => 6,
            Self::Adventure => 7,
            Self::Sports => 8,
            Self::Comedy => 9,
            Self::Western => 10,
            Self::Military => 11,
            Self::Building => 13,
            Self::FPS => 14,
            Self::RPG => 15,
        }
    }
}

/// Information comes directly from here <https://create.roblox.com/docs/studio/catalog-api#marketplace-api>.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct AvatarSearchQuery {
    /// Category must be filled to query more than one page.
    pub category: Option<Category>,
    /// Search by creator name. If `creator_type` is not provided, search is for users only.
    pub creator_name: Option<String>,
    /// Corresponds to a user id or group id depending on the creator type.
    /// Must be filled if `creator_type` is filled.
    pub creator_id: Option<u64>,
    /// Must be filled if `creator_id` is filled.
    pub creator_type: Option<CreatorType>,
    /// The genres of the item; keep in mind [`QueryGenre`] is different from [`Genre`].
    pub query_genres: Vec<QueryGenre>,
    /// The keyword to search for.
    pub keyword: Option<String>,
    /// The sort aggregation is used to sort the results by a specific metric.
    /// View [`SortAggregation`] for more information.
    pub sort_aggregation: Option<SortAggregation>,
    /// The sort type is used to sort the results in a specific order.
    /// View [`SortType`] for more information.
    pub sort_type: Option<SortType>,
    /// Subcategory must be filled to query more than one page.
    pub subcategory: Option<Subcategory>,
    /// The minimum price for each asset.
    pub min_price: Option<u32>,
    /// The maximum price for each asset.
    pub max_price: Option<u32>,
    /// The maximum assets Roblox should return per page.
    /// View [`Limit`] for more information.
    pub limit: Option<Limit>,
    /// Sort between different sale types of assets.
    /// View [`SalesTypeFilter`] for more information.
    pub sales_type_filter: Option<SalesTypeFilter>,
}

impl AvatarSearchQuery {
    /// Converts the query into a url.
    pub fn to_url(&self) -> String {
        let mut url = String::from(AVATAR_CATALOG_SEARCH_BASE_URL);

        if let Some(category) = self.category {
            url.push_str(&format!("category={}&", category.as_u8()));
        }

        if let Some(creator_name) = &self.creator_name {
            url.push_str(&format!("creatorName={}&", creator_name));
        }

        if let Some(creator_id) = self.creator_id {
            url.push_str(&format!("creatorTargetId={}&", creator_id));
        }

        if let Some(creator_type) = self.creator_type {
            url.push_str(&format!("creatorType={}&", creator_type.as_u8()));
        }

        if !self.query_genres.is_empty() {
            url.push_str("genre=");
            for query_genre in &self.query_genres {
                url.push_str(&format!("{},", query_genre.as_u8()));
            }
            url.push('&');
        }

        if let Some(keyword) = &self.keyword {
            url.push_str(&format!("keyword={}&", keyword));
        }

        if let Some(sort_aggregation) = self.sort_aggregation {
            url.push_str(&format!("sortAggregation={}&", sort_aggregation.as_u8()));
        }

        if let Some(sort_type) = self.sort_type {
            url.push_str(&format!("sortType={}&", sort_type.as_u8()));
        }

        if let Some(subcategory) = self.subcategory {
            url.push_str(&format!("subcategory={}&", subcategory.as_u8()));
        }

        if let Some(min_price) = self.min_price {
            url.push_str(&format!("minPrice={}&", min_price));
        }

        if let Some(max_price) = self.max_price {
            url.push_str(&format!("maxPrice={}&", max_price));
        }

        if let Some(limit) = self.limit {
            url.push_str(&format!("limit={}&", limit.as_u8()));
        }

        if let Some(sales_type_filter) = self.sales_type_filter {
            url.push_str(&format!("salesTypeFilter={}&", sales_type_filter.as_u8()));
        }

        // Remove the last & if it exists.
        if url.ends_with('&') {
            url.pop();
        }

        url
    }
}

/// A builder for [`AvatarSearchQuery`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct AvatarSearchQueryBuilder {
    query: AvatarSearchQuery,
}

impl AvatarSearchQueryBuilder {
    /// Creates a new `AvatarSearchQueryBuilder`.
    pub fn new() -> Self {
        Self {
            query: AvatarSearchQuery::default(),
        }
    }

    /// Builds the [`AvatarSearchQuery`].
    pub fn build(self) -> AvatarSearchQuery {
        self.query
    }

    #[allow(missing_docs)]
    pub fn category(mut self, category: Category) -> Self {
        self.query.category = Some(category);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_name(mut self, creator_name: String) -> Self {
        self.query.creator_name = Some(creator_name);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_id(mut self, creator_id: u64) -> Self {
        self.query.creator_id = Some(creator_id);
        self
    }

    #[allow(missing_docs)]
    pub fn creator_type(mut self, creator_type: CreatorType) -> Self {
        self.query.creator_type = Some(creator_type);
        self
    }

    #[allow(missing_docs)]
    pub fn query_genres(mut self, query_genres: Vec<QueryGenre>) -> Self {
        self.query.query_genres = query_genres;
        self
    }

    #[allow(missing_docs)]
    pub fn keyword(mut self, keyword: String) -> Self {
        self.query.keyword = Some(keyword);
        self
    }

    #[allow(missing_docs)]
    pub fn sort_aggregation(mut self, sort_aggregation: SortAggregation) -> Self {
        self.query.sort_aggregation = Some(sort_aggregation);
        self
    }

    #[allow(missing_docs)]
    pub fn sort_type(mut self, sort_type: SortType) -> Self {
        self.query.sort_type = Some(sort_type);
        self
    }

    #[allow(missing_docs)]
    pub fn subcategory(mut self, subcategory: Subcategory) -> Self {
        self.query.subcategory = Some(subcategory);
        self
    }

    #[allow(missing_docs)]
    pub fn min_price(mut self, min_price: u32) -> Self {
        self.query.min_price = Some(min_price);
        self
    }

    #[allow(missing_docs)]
    pub fn max_price(mut self, max_price: u32) -> Self {
        self.query.max_price = Some(max_price);
        self
    }

    #[allow(missing_docs)]
    pub fn limit(mut self, limit: Limit) -> Self {
        self.query.limit = Some(limit);
        self
    }

    #[allow(missing_docs)]
    pub fn sales_type_filter(mut self, sales_type_filter: SalesTypeFilter) -> Self {
        self.query.sales_type_filter = Some(sales_type_filter);
        self
    }
}
