use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    let interactions = vec![
        "Come no closer, ghost.\nI've seen you, creeping through the undergrowth, stalking me.\nThis old kingdom... A terrible thing awakens. I can smell it in the air...\nI know what you are. I know what you'd try to do. I can't allow it. ",
        "Again we meet little ghost.\r\rI'm normally quite perceptive. You I underestimated, though I've since guessed the truth.\rYou've seen beyond this kingdom's bounds. Yours is resilience born of two voids.\rIt's no surprise then you've managed to reach the heart of this world. In so doing, you shall know the sacrifice that keeps it standing.\rIf, knowing that truth, you'd still attempt a role in Hallownest's perpetuation, seek the Grave in Ash and the mark it would grant to one like you.",
        "So you'd pursue the deeper truth? It isn't one the weak could bear.\r\rProve yourself ready to face it. I'll not hold back. My needle is lethal and I'd feel no sadness in a weakling's demise.\rShow me you can accept this Kingdom's past and claim responsibility for its future. ",
        " \t...So strong...\r\rYou could do it, if you had the will.\rBut could you raise your nail once knowing its tragic conception? And knowing yourself?...\rThen do it, Ghost of Hallownest! Head onward. Burn that mark upon your shell and claim yourself as King. ",
        "Ghost. I see you've faced the place of your birth, and now drape yourself in the substance of its shadow.\r\rThough our strength is born of similar source, that part of you, that crucial emptiness, I do not share.\rFunny then, that such darkness gives me hope. Within it, I see the chance of change.\rA difficult journey you would face, but a choice it can create. Prolong our world's stasis or face the heart of its infection. ",
        "I'd urge you to take that harder path, but what end may come, the decision rests with you. ",
        "...It faced the void, and ascends unscathed... Could it unite such vast darkness?.. ",
        "So you've slain the Beast... and you head towards that fated goal.\r\rI'd not have obstructed this happening, but it caused me some pain to knowingly stand idle.\r...What? You might think me stern but I'm not completely cold.\rWe do not choose our mothers, or the circumstance into which we are born. Despite all the ills of this world, I'm thankful for the life she granted me.\rIt's quite a debt I owed. Only in allowing her to pass, and taking the burden of the future in her stead, can I begin to repay it. ",
        "Leave me now, ghost. Allow me a moment alone before this bedchamber becomes forever a shrine. ",
        " 	...Mother... Forgive my inaction... but another path may be possible... ",
        "I'm impressed little ghost. You've burdened yourself with the fate of this world, yet you still stand strong.\r\rTo break the Dreamer's seals would alone be considered an impossible task, but to accept that void inside yourself, that casts you as something rather exceptional. ",
        "The path is opened. One way or another an end awaits inside.\r\rI won't be joining you in this. That space is built to sustain your likes. Its bindings would drain me were I to join.\rDon't be surprised. I'll not risk my own life in your attempt, though if the moment presents I'll aid as I'm able. ",
        " 	Ghost of Hallownest, you possess the strength to enact an end of your choosing. Would you supplant our birth-cursed sibling, or would you transcend it? ",
        "...Could it achieve that impossible thing? Should it? ",

    ];

    let mut rng = thread_rng();

    if let Some(random_string) = interactions.choose(&mut rng) {
        random_string.to_string()
    } else {
        "Sorry traveler, i have nothing to say".to_string()
    }


}

pub fn register() -> CreateCommand {
    CreateCommand::new("hornet").description("Trigger a random interaction with Hornet.")
}