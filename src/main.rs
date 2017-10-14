extern crate serenity;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate spin;
#[macro_use]
extern crate lazy_static;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use std::env;
use std::collections::HashMap;
use spin::Mutex;

#[derive(Deserialize)]
struct Config {
    token: String,
    user_roles: Vec<String>,
    log_channel: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = {
        let mut path = "$HOME/.config/sudobot/config.toml";
        let mut f = File::open(path).unwrap();

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let cfg: Config = toml::from_str(&buffer).unwrap();

        Mutex::new(cfg)
    };
}


struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and serving {} servers.", ready.user.name, ready.guilds.len());
    }

    fn on_guild_ban_addition(&self, _: Context, guild_id: GuildId, user: User) {
        let cache = CACHE.read().unwrap();

        if let Some(g) = cache.guild(guild_id) {
            //Grab a Guild object to play with.
            let mut guild = g.read().unwrap();

            let log_channel = format!("#{}", CONFIG.lock().log_channel);

            let channels = &guild.channels;

            for value in channels.values() {
                let mut name = &value.read().unwrap().name;

                match name {
                    log_channel => {
                        let mut id = &value.read().unwrap().id;

                        let mut bans = guild.bans().unwrap();

                        for ban in bans {
                            //Check if the current element of the iterator matches, and if it does
                            //we can access more info about the ban.
                            if ban.user == user {
                                match ban.reason {
                                    Some(r) = {
                                        let user_discrim = format!("{}{}", user.name, user.discriminator);

                                        if let Err(err) = id.say("User {} was banned for reason {}", user_discrim, r) {
                                            println!("Error sending message to channel {} for reason {}", id, err);
                                        }
                                    }

                                    None {
                                        let user_discrim = format!("{}{}", user.name, user.discriminator);

                                        if let Err(err) = id.say("User {} was banned. No reason given.", user_discrim) {
                                            println!("Error sending message to channel {} for reason {}", id, err);
                                        }
                                    }
                                }
                            }
                        }
                    },

                    _ => {
                        //Tell the server owner that we couldn't find the correct log channel.
                        let mut owner_id = guild.owner_id;

                        for member in guild.members.values() {
                            //Get a user object to use.
                            if member.user.read().unwrap().id == owner_id {
                                let user = member.user.read().unwrap();
                                
                                let info_string = format!("This server does not have a log channel as configured, please make sure that the channel exists and this bot has permission to access it");

                                //Try to dm the server owner that this didn't work.
                                match user.dm(|m| m.content(&info_string)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("Error sending log info: {}", e);
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}


fn main() {
    
}
