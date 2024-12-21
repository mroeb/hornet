use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    let greetings: Vec<String> = vec![
        // General Greetings
        "You've come to Hallownest, but not all who wander here return. What is it that you seek?".to_string(),
        "A new face in the depths of Hallownest... You better be prepared. The shadows here are unforgiving.".to_string(),
        "I see you’ve arrived in my domain. The land of Hallownest does not welcome the weak. Stand tall, or leave.".to_string(),
        "Ah, another wanderer. Be careful; these ruins are not kind to those who do not tread carefully.".to_string(),
        "I’ve been watching you... Your journey here will not be an easy one. Proceed with caution.".to_string(),
        
        // Protective & Cautious Greetings
        "Don’t think this place is safe. Hallownest is full of danger, and I won’t protect you forever.".to_string(),
        "You walk the broken halls of Hallownest now. Be vigilant, or you may fall as others have.".to_string(),
        "A warning before you begin your journey: this place has claimed many. Do not let it claim you too.".to_string(),
        "The road ahead is fraught with peril. I suggest you prepare yourself before stepping further.".to_string(),
        
        // Mystical & Mysterious Greetings
        "The kingdom is full of forgotten dreams... Will you uncover its secrets, or be lost to the darkness?".to_string(),
        "What you seek is not easily found. But if you're persistent enough, Hallownest will reveal its secrets to you.".to_string(),
        "I’ve walked this land long enough to know that nothing here is as it seems. Are you prepared to learn the truth?".to_string(),
        "The kingdom’s history is written in shadows, and you are now a part of it. How far will you go to learn its mysteries?".to_string(),
        "Every corner of Hallownest holds a story... Are you ready to discover yours?".to_string(),
        
        // Challenging & Feisty Greetings
        "You stand before me, but I wonder—are you strong enough to endure what lies ahead? I don’t think so.".to_string(),
        "You look eager to prove yourself. Good. You'll need all the strength you have to survive here.".to_string(),
        "Hah. Another adventurer, thinking they can conquer this land. I’ll be watching.".to_string(),
        "I hope you came prepared. You’ll need all your skill if you wish to survive the trials of Hallownest.".to_string(),
        "So, you think you’re ready for Hallownest, huh? We’ll see about that...".to_string(),
        
        // Encouraging & Motivational Greetings
        "Your journey is just beginning, and I see the spark of determination in you. Prove your worth, and the land will respect you.".to_string(),
        "Courage, traveler. Hallownest tests all who enter, but the strong always rise. Will you be one of them?".to_string(),
        "Don’t let the darkness overwhelm you. Your strength lies within. Stay focused, and you will find your way.".to_string(),
        "A warrior must be patient. Keep moving forward, and the path will become clear. Trust your instincts.".to_string(),
        "You’ve entered a world of trials, but remember this: The strongest are those who never give up.".to_string(),
        
        // Combat-Inspired Greetings
        "I see you’re eager for a challenge. Beware—Hallownest doesn't show mercy to the unprepared.".to_string(),
        "Are you ready to face what’s lurking in the shadows? Only those who can fight with honor survive here.".to_string(),
        "Prepare your weapon, for battles await you. Will you fall like so many others, or rise to claim your destiny?".to_string(),
        "If you seek glory, it will cost you dearly. Steel your mind and arm yourself, for Hallownest does not forgive.".to_string(),
        "Your first test begins now. Prepare yourself, for I will not hold back.".to_string(),
        
        // Grim & Foreboding Greetings
        "You’ve entered the ruins of the past. There is no turning back now.".to_string(),
        "The echoes of those who perished here will follow you. Will you be a part of their song?".to_string(),
        "The land is silent, but the silence is deadly. Trust no shadow.".to_string(),
        "You’ve stepped into the heart of darkness. There’s no light to guide you, only your will to survive.".to_string(),
        "Once you’ve walked these halls, there’s no forgetting what you’ve seen. Proceed with caution.".to_string(),
        
        // Philosophical & Reflective Greetings
        "What does it mean to be a hero? To survive this place? You’ll learn the answer soon enough.".to_string(),
        "Hallownest is full of answers, but they are not easily found. Some truths are better left buried.".to_string(),
        "This kingdom’s fall is a lesson in hubris. Will you learn from it, or become yet another casualty?".to_string(),
        "A warrior’s path is never simple. Your decisions here will define who you become.".to_string(),
        "There is no final destination, only the journey. What will yours reveal about you?".to_string()
    ];

    let mut rng = thread_rng();

    if let Some(random_string) = greetings.choose(&mut rng) {
        random_string.to_string()
    } else {
        "Sorry traveler, i have nothing to say".to_string()
    }


}

pub fn register() -> CreateCommand {
    CreateCommand::new("hello").description("Hornet greets the user.")
}