use account_curve_filter::classify::classify_pubkeys;

#[tokio::main]
async fn main() {
    // Addresses to classify
    let addresses: Vec<&str> = vec![
        "86AaKs3tJnuMPB9ZrcdaH1PyZhxXnHjXxfPHQrkeEfMH",
        "EWF5rAB7kuw6jWcg4m7GTPBdh4W1TpQGNKKL9RcZ7Asp",
        "vVGxiLvYjLv8Jvpic9yMbR291s7CD6HcQxdW6QzT3jL",
        "6qQNZRcZefNtfL3ZFSLpbDqsuHmNKCt1ADgKPnDF6uFL",
        "CmAhDf2JH6nTvBPgg5p2FwBnhBGELEbmULAqtQFio36b",
        "ASTyfSima4LLAdDgoFGkgqoKowG1LZFDr9fAQrg7iaJZ",
        "5Hr7wZg7oBpVhH5nngRqzr5W7ZFUfCsfEhbziZJak7fr",
        "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
        "6ne5e9QVDM6NkWY4dU8KxUWnFAf7x78ZTac4pCSuUfoQ",
        "3LoAYHuSd7Gh8d7RTFnhvYtiTiefdZ5ByamU42vkzd76",
        "5hvysTJqsbR6vFkYrFq15QdBK7D95Zdacefnry1DVVkP",
        "F5GkEY9zmFLaDtrJZAu8oojgczy2KW1siCtT1GDK8Yru",
        "JAhTBr6n15Srt8pRUfA5f2jzi3wh47PQbVZhXRaB6dWk",
        "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
        "AtSFyzUcwKgWRVsvLDi4rNfQVdHTXWYkJx2nniZfLjBj",
        "AeNbpxUKnuSxg7czcizUkqghojoaxwkDzXV6s55NSC4t",
        "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",
        "2byKurzc6rBe8ucWQ7sRdDS1fzC4Qtsia6Nwg66fbUn9",
        "N5QXCmRjQaBLR5RjMeTziSaH2ZR852tAzMSmVjptJ3M",
        "dustFPTV7dujoJjgkKtf6is3bYaFEy1nswS23vxHfvt",
        "FERjPVNEa7Udq8CEv68h6tPL46Tq7ieE49HrE2wea3XT",
        "5zvQSxBbN6ZKd7susVy6Vbt7QkDLusWwST93vHkFEUme",
        "7iWnBRRhBCiNXXPhqiGzvvBkKrvFSWqqmxRyu9VyYBxE",
        "HWayWyoAzLTr9s43eqJ7VcmEX3H3KAA8RMmYfDzQwGGT",
        "GugU1tP7doLeTw9hQP51xRJyS8Da1fWxuiy2rVrnMD2m",
        "92yZbmmppHwZYAPovJmPZaofR8tn2cUyVJhD5XLCJKP5",
        "HTvjzsfX3yU6BUodCjZ5vZkUrAxMDTrBs3CJaq43ashR",
        "HSCWtxXrPRXUJoEziEjWJvXMGZ4its5eTBaUkgGvZoom",
        "Ay8xzWMqeDSyqDp6A6vzYisVbcfF6XcDMZTRt8VGdQup",
        "HPKFd21TU88twq7gNiwxp5UsTyykeiFu3JtzZ4VRgw1V",
        "394iV6GMbpAr1VFMQJuFyLfPg3iw54ZohBqUsY8mGRfL",
        "KdK5wWfELMjp5V3igRRLwHK7sWUGSaFp7x4c8LmB9s2",
        "HFqp6ErWHY6Uzhj8rFyjYuDya2mXUpYEk8VW75K9PSiY",
        "4xDsmeTWPNjgSVSS1VTfzFq3iHZhp77ffPkAmkZkdu71",
        "HU23r7UoZbqTUuh3vA7emAGztFtqwTeVips789vqxxBw",
        "GGztQqQ6pCPaJQnNpXBgELr5cs3WwDakRbh1iEMzjgSJ",
        "2H6fxbjyzKzdUxX9hNqHP36J5tq6921to9kBRTLMWiQr",
        "Bn9TwxLHhP3cV6oke3pCgXF81ijYkSZ69mhZwrVHq9TT",
        "Ez2U27TRScksd6q7xoVgX44gX9HAjviN2cdKAL3cFBFE",
        "DCAKuApAuZtVNYLk3KTAVW9GLWVvPbnb5CxxRRmVgcTr",
        "AkNk3gxMfJjRrw7fTnwSDurgXzzXGLGd1hun1R77f71R",
        "CM5Gpe95BJDkX6CeewKqu4bhTAeu4gkGv5UkgTw9ahT9",
        "E2Db5PEF28m4edGvx5TpR6UWJRKyJbefny65FK9wPWVg",
        "26y9VLV6Ekxedwib6oLjkLTDMEoZ1x7SLME2oVeq68LZ",
        "3CgvbiM3op4vjrrjH2zcrQUwsqh5veNVRjFCB9N6sRoD",
        "GrzVxTPmRiSdigd6SJMWMyjfRXXAk874mphrS6VjURqp",
        "DxCqcDWPPhYdnKPETUMEQpTFWTATEvikFfuckJTKAWMd",
        "GioSL3DAVUVor4euvhUqMseqT3jCnSYQ7ZxKJC1hZhvo",
        "FSTdP2osbfKtF9FT8pe4igsqYzGzmhUMVQNSrV2wvnDV",
        "46JyATskL6Ab65PwjNpn8akiEKSA6SoMrAHSrTb4kKnb",
        "A8MivGfaY3oUkfgdhJkPucBwyzZd13yQ9ABHyzrQuvqs",
        // Añade más direcciones aquí",
    ];

    // Classify addresses
    let (on_curve, off_curve) = classify_pubkeys(addresses);

    println!("On-curve public keys: {:?}", on_curve);
    println!("Off-curve public keys: {:?}", off_curve);
}
