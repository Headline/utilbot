// Utilbot - A random utility bot
// Copyright (C) 2020  Michael Flaherty
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[derive(Default)]
pub struct PlayerStats {
    pub total : i32,
    pub attack : i32,
    pub defense : i32,
    pub strength : i32,
    pub hp : i32,
    pub ranged : i32,
    pub prayer : i32,
    pub magic : i32,
    pub cooking : i32,
    pub woodcutting : i32,
    pub fletching : i32,
    pub fishing : i32,
    pub firemaking : i32,
    pub crafting : i32,
    pub smithing : i32,
    pub mining : i32,
    pub herblore : i32,
    pub agility : i32,
    pub theiving : i32,
    pub slayer : i32,
    pub farming : i32,
    pub runecrafting : i32,
    pub hunter : i32,
    pub construction : i32,
}

impl PlayerStats {
    pub async fn new(playername : &str) -> PlayerStats {
        let formatted_playername = playername.replace(" ", "%20");
        let url = format!("https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}", formatted_playername);
        println!("Sending request to: {}", url);
        let res = reqwest::get(&url).await;
        if res.is_err() {
            return PlayerStats {
                ..Default::default()
            }
        }
        let result = res.unwrap();
        let body = result.text().await.unwrap();
        let mut levels = Vec::new();
        let mut count = 0;
        for line in body.split("\n") {
            let level = line.split(",")
                .map(|x| x.parse::<i32>().unwrap_or_else(|_| 0))
                .collect::<Vec<i32>>()[1];
            levels.push(level);
            count+=1;
            if count > 23 {
                break;
            }
        }

        PlayerStats{
            total:          levels[0],
            attack:         levels[1],
            defense:        levels[2],
            strength:       levels[3],
            hp:             levels[4],
            ranged:         levels[5],
            prayer:         levels[6],
            magic:          levels[7],
            cooking:        levels[8],
            woodcutting:    levels[9],
            fletching:      levels[10],
            fishing:        levels[11],
            firemaking:     levels[12],
            crafting:       levels[13],
            smithing:       levels[14],
            mining:         levels[15],
            herblore:       levels[16],
            agility:        levels[17],
            theiving:       levels[18],
            slayer:         levels[19],
            farming:        levels[20],
            runecrafting:   levels[21],
            hunter:         levels[22],
            construction:   levels[23]
        }
    }
}