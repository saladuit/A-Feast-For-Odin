// https://docs.rs/bevy/latest/bevy/hierarchy/index.html
// commands.spawn((Player, ...)).with_chidren(|parent| parent.spawn((Viking, ...)));
// I have an parent child question. Is it advisable just to rely on the parent children feature completely, instead of using player id's? In my case a player has workers, a player board, cards, goods, resources, islands and buildings
// Needed Components
// Player { ID, Supply, Score, HomeBoard } # ( Max 4 players )
// Score
// Supply { Wood, Stone, Ore, Silver, WeaponCards, OccupationCards, Goods, SpecialTiles, ThingPenalties } # OccupationCards can be in supply or in hand
// Goods { GoodsColour, GoodSize, Option(SwordValue), Option(HarvestNumber), Option(Animal)}
// GoodsColour { Orange="Farm Product", Red="Animal Product", Green="Craft Product", Blue="Luxury Good" }
Unlimited Supply

// Animal { Pregnant/non-pregnant }
// Viking { Colour, VikingState }
// VikingState { BanquetTable, ThingSquare, ActionBoard }
// Dice { DiceSides }
// DiceSides { Eight, Twelve, Twenty }
// ActionBoard -> ActionSpace -> ActionGroup, ActionColumn
// HomeBoard -> PlacementArea {Income Diagonal, Goods Spaces}, ThingSquare{Idle Vikings}, Stables{Sheep, Cattle}, BanquetTable, Landing Stages { 3x small, 4x large }, Waiting Space (solo game)
// BanquetTable { 6xBanquetTableSpace }
// BanquetTableSpace { 2xFeastSpace, BanquetTableState }
// BanquetTableState { Emigrated, NotEmigrated}
// FeastSpace { FeastSpaceState }
// FeastSpaceState { Covered, Uncovered }
// ShipSupplyBoard { WhalingBoatSpace, KnarrSpace, LongshipSpace }
// WhalingBoat { points: 3, cost: 3, size: Small }
// Knarr { points: 5, cost: 5, size: Large }
// Longship { points: 8, cost: 8, size: Large }
// Armable { max_arming, current_arming }
// Weapon Card -> WeaponType { Sword, Bow, Spear, Snare }
// ExplorationBoards -> ExplorationBoardType { A, B, C, D}, ExplorationBoardSide { Front, Back }, IncomeDiagonal, ExplorationBoardState { Unclaimed, Claimed }
// BonusTiles
// IncomeDiagonal { size, lowest_visible_number }
// Resources -> Wood, Stone, Ore, Silver
// OvalSupplyBoard { Special Tiles }
// Special Tiles -> Name, Spaces, Sword Value, Silver Cost, Forge Symbol -> { 
// { Glass Beads, 5, 7, 0, false} ,
// [ Helmet, 5, 8, 1, true } ,
// { Cloakpin, 5 8, 1, true} ,
// { Belt, 5, 8, 2, false} ,
// { Crucifix, 6, 8, 2, true} , 
// { Drinking Horn, 6, 8, 2, false} ,
// { Amber Figure, 7, 9, 2, false} ,
// { Horseshoe, 7, 9, 2, true} ,
// { Gold Brooch, 8, 9, 3, false} ,
// { Forge Hammer, 9, 10, 4, true} ,
// { Fibula, 9, 10, 4, true} ,
// { Throwing Axe, 9, 11, 4, true} ,
// { Chalice, 10, 12, 5, false} ,
// { Crown English, 12, 13, 6, false} ,
// { Crown, 13, 16, None, false} , # Also Don't forget to score 2 points
// Round Overview
// Round Tracker
// Building Tiles -> BuildingType -> Shed, Stone House, Long House
// Mountain Strips {7x resources{ Wood, stone ore, silver}
// Starting Player Moose
// Occupations Deck -> DeckVersion { A, B, C} -> DeckType { Starting, Dark }
// Occupation Card => Occupation Points, OccupationType { Each Time, Immediate, Anytime, As Soon As }, Description
// Thing Penalty
// Extention Tiles -> ExtationTileSide { Front, Back } -> ExtentionTileVersion { Left, Right }
// Emigrations

// Needed Systems
// 0 Configure 
// 0.1 Choose a occupation Deck
// 0.2 Game Length
// 0.3 Choose Starting Islands
// 0.4 Enable Mini Expansion
// 1 Setup -> 
// 1.1 Load the Action Board
// 1.2 the 'Goods Boxes', 
// 1.3 the Extension tiles
// 1.4 Special Tiles Supply Board
// 1.5 Ship Supply Board
// 1.6 two or three (4-player game) mountain strips
// 1.7 the exploration Boards “Shetland”, “Faroe Islands”, “Iceland”, and “Greenland” sides facing up in that order.
// 1.8 Place the “shed”, “stone house”, and “long house” building tiles
// 1.9 the “round overview” and “round tracker” boards
// 1.10 Shuffle the Occupation Cards and Weapon Cards, remaining mountain strips
// 1.11 Draw a Starting Occupation, one mead, a bow, net and spear for Each Player
// 1.12 Roll for starting player
// 2 Phases
// 2.1 A New Viking -> Add a viking to the thing square
// 2.2 Harvest -> Receive orange goods according to the harvest number(s)
// 3 Turn Exploration Boards and Placing Silver
// 3.1 In round 3, turn A, in round 4 turn B, in round 5 turn C, and in round 6 turn D
// 3.1.1 Reset silver when a board is turned with silver on it
// 3.2 Place 2 silver on each unclaimed exploration board not being turned
// 4 Draw a New Weapon
// 4.1 Place it face up  in their supply
// 5 Actions
// 5.1 Beginning with the start player and in clockwise order, take one or more Vikings from tyour thing Square and place them on exactly one unoccupied action space of the action board
// 5.2 You must use at least one of its effects (Paying a cost is also considered an effect)
// 5.3 Each action space can only be occupied once per round
// 5.4 You must use an action space immediatly
// 5.5 Continue until all players have passed or placed all of their available Vikings
// 6 determine Start Player
// 6.1 The player who placed vikings last in the action pahse is the start player
// 6.2 receives the moose
// 7 Income
// 7.1 The smallest uncovered income diagonal indicates the amount of silver you receive this phase
// 7.2 Must be covered in ascending diagonal order p12
// 7.3 exploration boards can produce additional income
// 8 Animal Breeding
// 8.1 If a player has at least 1 animal that is pregnant, turn it and take 1 corresponding animal
// 8.2 If a player has at least 1 animal that is not pregnant, turn it
// 9 Feast
// 9.1 Place orange and red food tiles from your supply (or stable) and/or "1 silver" coins on each empty space of the banquet table
// 9.1.1 Orange tiles cannot be placed next to eachother
// 9.1.2 Red tiles also cannot be placed next to each other
// 9.1.3 Silver can be placed next to each other
// 9.1.4 One tile of each type MAY be placed horizontally
// 9.1.5 All other tiles of that type MUST be placed vertically
// 9.1.6 Every tile you place on the Banquet Table must cover at least one of the squares below the table and fit in the allowed space.
// 9.1.7 For each uncovered space you receive a ThingPenalty and add it to your supply
// 9.1.8 Move goods to general supply
// 10 Bonus Tiles
// 10.1 If you leave a goods space uncovered but instead cover the 8 spaces around it you will receive that good.
// 10.2 Players receive the bones at the same time
// 11 Update and add New mountain Strips
// 11.1 Remove the leftmost building resource from each face-up mountain strip
// 11.2 If all spaces but the last "2 silver" spae are empty remove the two silver and the entire mountain strip
// 11.3 Turn a new mountain strip face up and place the depicted goods on it
// 11.4 In a 4-player game there are not enogh mountain strips. You simply do not add in round 7.
// 12.1 Return Vikings from the action board to your Thing Square
// 12.2 Continue to phase 1 of the next round

// Any Time Actions
// 1. Placing Goods on the Home Board
// 1.1 You can only cover a space in the income diagonal if you also cover all spaces:
// 1.1.1 to the left of that space
// 1.1.2 below that space
// 1.1.3 all the spaces in the square to the bottom left of that space
// 1.1.4 Spaces with goods are considered covered already
// 1.2 Green tiles cannot be orthogonally adjacent to other green tiles
// 1.3 Blue tiles, silver and ore can be adjacent to any other tile
// 1.4 You cannot place goods on top of other goods
// 1.5 You can place goods over spaces with goods symbols
// 1.6 Each good must be placed withing the bounds of the placement area
// 1.7 Special cards are considered blue goods
// 2. Placing goods on the Exploration Boards
// 2.1 You may place green and blue goods, silver coins and ore on empty spaces according to the same rules
// 2.2 You cannot cover green border space
// 3. Placing goods on the House Tiles
// 3.1 You may place Orange, red, green and blue goods, silver coins on empty spaces.
// 3.2 Orange tiles cannot be orthogonally adjacent to other orange tiles,
// 3.3 Red tiles cannot be orthogonally adjacent to other red tiles
// 3.4 You cannot cover pillar places
// 4. Once you place a goods tile on a board, island or house you cannot take it back
// 4.1 Testing tile placements is allowed
// 5 Buying Ships
// 5.1 You may buy a ship by paying its cost in silver
// 5.2 The cost of a ship is equal to its value
// 5.3 Each ship you acquir emust be placed in your bay on a landing stage of appropriate size
// 5.3.1 The whaling boats are small
// 5.3.2 The knarrs and drakars are large
// 6. Changing Silver
// 6.1 Silver ccomes in denominations of "1 silver", "2 silver", "4 silver" and "10 silver"
// 7. Arming
// 7.1 Before but not during an action you may place ore from your supply on the designated spaces of your whaling boats and longships
// 8. Occupations
// 8.1 Activate an any time occupation card

// Production Spaces
// 1. Green spaces give what is depicted
// 2. If a good costs silver the price is printed below a purse icon
// Exchange Spaces
// 1. Exchange something for something else, but only once
// Mountain and Trading Actions
// 1. Some spaces allow for taking building resources and silver from mountain strips
// 1.1 alwasy take from left to right
// 1.2 "2 silver" is considered single item
// 1.3 Take 2 building resources allows you to take up to 2 items from one mountain strip
// 1.4 Take 3 + 2 allows you to take up to 3 from one strip and up to to from another
// 1.5 2+2+2+2 also needs 4 different strips
// Trading Actions
// 1. The ^ 1 good action allows you to exchange exactly 1 good tile for the next more valuable one
// 1.1 either orange to red, red to green, green to blue
// 1.2 the old and new tile must have the same shape and dimensions
// 2. the ^ 2 goods action allows you to upgrade 2 goods
// 2.1 you may bot upgrade a single tile twice
// 3. the ^^ 2 goods action allows you to upgrade 2 goods twice
// 3.1 you may exchange an orange good for a green one, or a red good for a blue one

// Bue stands for overseas trading
// Yellow stands for emigration
// Red stands for hunting, raiding and pillaging
// Brown is occupation cards

// Overseas Trading
// 1. Pay 1 silver to turn any number of different green goods tiles to their blue side
// 1.1 you must have one knarr in your bay
// 1.2 Green tiles in the placement areas of your boards may not be turned
// Special Sale
// 1. If you have at least one knarr you can buy upt to 2 special tiles by paying their cost
// 1.1 Except for the english crown
// Emigration
// 1. You may turn a Knarr or a longship in your possession to its back side
// 1.1 Place it on the leftmost space of your Banquet Table
// 1.2 This tile is no longer a ship
// 1.3 the action reduces the amount of food you have to serve during a feast
// 1.4 You must pay an amount of silver equal to the current round
// 1.5 You must have space for another ship on your banquet table to take this action
// 1.6 You lose the ore on the longship if any

// Hunting, Raiding and Pillaging
// 1. When raiding and pillaging you want to roll high
// 2. When hunting you want to roll low
// 3. You may roll the die up to three times
// 3.1 You can stop rolling at any time
// 3.2 Each re-roll invalidates the previous roll
// 4. Announce your battle result
// 5. Decalre your die action a success or failure

// Raiding
// 1. You need at least one Drakkar and want to roll high
// 2. It doesn't matter if there is ore on the ship
// 2.1 You roll with the red eight-sided die
// 3. Success
// 3.1 Every blue goods tile has a sword value.
// 3.2 After you roll the orange eight-sided die
// you can take exactly one blue goods tile with a sword value equal to or lower than you rolled
// 3.3 Each stone and long sowrd weapon card you spend increases your die rol by 1
// 3.4 this modified value is your battle result
// 4. Failure
// 4.1 Your raid automatically fails if your battle result is 5 or less
// 4.2 Regardless of the result you may willingly decalre your raid a failure
// 4.3 Upon failure you receive a consolatory 1 stone and 1 long sword

// Pillaging
// 1. You need at least one Drakkar
// 1.1 If you have more than one drakkar you must use the longship with the most ore on it
// 1.2 you roll with the blue twelve-sided die
// 2.1 Add 1 to your dice roll for each ore on your drakkar
// 2.1.1 you keep the ore
// 2.2 After you roll you can take exactly one blue goods tile witha sword value equal to or lower than you rolled
// 2.3 You can spend stone and long swords from your supply to increase your die roll by 1
// 2.4 this modified value is your battle result

// Hunting Game, Laying a Snare and Whaling
// 1.1 When hunting game and laying a snare you roll the orange eight sided die.
// 1.2 When whaling you roll the blue twelve sided die
// 1.3 In all these cases you want to roll low
// 2. Success
// 2.1 When hunting laying snares and whaling
// reduce the value of your roll by one for each ore (even preprinted) on the whaling boats.
// 2.2 This can lead to a negative value
// 2.3 any negative value is treated as a zero
// 2.4 Pay a number of wood or proper weapons equal to this value to declare a successful action
// 2.5 receive the reward depicted on the action space
// 3. Failure
// 3.1 If you declare your hunting attempt a failure
// you do not have to pay anything
// 3.1.1 You may not declare a result of 0 a failure
// 3.2 Upon failure you receive a wood and 1 weapon of the proper type
// 3.2.1 additionally, when you fail laying a snare you may
// return 1 Viking from the action space to your Thing Square
// 3.2.2 when you fail whaling you may even return 2 vikings

// Exploration
// 1. Orange is the color of exploration
// 2. When taking an exploration action
// 2.1 Place the required number of Vikings on the action space and take the corresponding exploration board
// 3. If there is silver on the board, place it in your supply
// 4. You keep the ship you explore with

// Occupation
// 1. You start with a single light brown occupation card
// 1.1 You keep occupation cards hidden in your hand

// Needed UI
// Visible weapon cards and tiles
// Round tracker

// #[derive(Component)]
// enum Colour {
//   Yellow,
//   Red,
//   Green,
//   Blue,
// }

// impl Colour {
//   fn upgrade(&self) -> Result<Colour, Error> {
//     match self {
//       Colour::Yellow => Ok(Colour::Red),
//       Colour::Red => Ok(Colour::Green),
//       Colour::Green => Ok(Colour::Blue),
//       Colour::Blue => Err("Error::ColourMaxLevelError"),
//     }
//   }
// }

// #[derive(Component)]
// struct Goods {
//   height: u8,
//   width: u8,
//   colour: Colour,
// }

// #[derive(Component)]
// enum DiceSides {
//   Eight,
//   Twelve,
// }

// #[derive(Component)]
// struct Dice {
//   sides: DiceSides,
// }

// #[derive(Component)]
// enum WeaponType {
//   Sword,
//   Bow,
//   Shield,
// }

// #[derive(Component)]
// struct Viking {
// }

// #[derive(Component)]
// struct Resources {
//   silver: Silver,
//   wood: Wood,
//   stone: Stone,
// }
// #[derive(Component)]
// struct Score {
//   amount: u32,
// }

// #[derive(Component)]
// struct WeaponCard {
//   weapon_type: WeaponType,
// }

// #[derive(Component)]
// struct Silver {
//   amount: u32,
// }

// #[derive(Component)]
// struct Wood {
//   amount: u32,
// }

// #[derive(Component)]
// struct Stone {
//   amount: u32,
// }

// #[derive(Component)]
// struct Player {
//   id: u8,
//   workers: Workers,
//   resources: Resources,
//   score: Score,
//   home_board: HomeBoard,
// }