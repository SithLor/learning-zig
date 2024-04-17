function CAL(month, year,scale){
    // may 2045
    // |Su |Mo |Tu |We |Th |Fr |Sa |
    // | 1 | 2 | 2 | 3 | 4 | 5 | 6 |
    // | 7 | 8 | 9 |10 |11 |12 |13 |
    // |14 |15 |16 |17 |18 |19 |20 |
    // |21 |22 |23 |24 |25 |26 |27 |
    // |28 |29 |30 |31 |   |   |   |
    const e = {
        row_1: `month: ${month} year: ${year}`,
        row_2: `|Su ${scale}|Mo ${scale}|Tu ${scale}|We ${scale}|Th ${scale}|Fr ${scale}|Sa ${scale}|`,
        row_3: `|${scale} 1 ${scale}|${scale} 2 ${scale}|${scale} 2 ${scale}|${scale} 3 ${scale}|${scale} 4 ${scale}|${scale} 5 ${scale}|${scale} 6 ${scale}|`,
        row_4: `|${scale} 7 ${scale}|${scale} 8 ${scale}|${scale} 9 ${scale}|${scale}10 ${scale}|${scale}11 ${scale}|${scale}12 ${scale}|${scale}13 ${scale}|`,
        row_5: `|${scale}14 ${scale}|${scale}15 ${scale}|${scale}16 ${scale}|${scale}17 ${scale}|${scale}18 ${scale}|${scale}19 ${scale}|${scale}20 ${scale}|`,
        row_6: `|${scale}21 ${scale}|${scale}22 ${scale}|${scale}23 ${scale}|${scale}24 ${scale}|${scale}25 ${scale}|${scale}26 ${scale}|${scale}27 ${scale}|`,
        row_7: `|${scale}28 ${scale}|${scale}29 ${scale}|${scale}30 ${scale}|${scale}31 ${scale}|${scale.repeat(3)}|`,
    }
    const final_data= `\n${e.row_1}\n${e.row_2}\n${e.row_3}\n${e.row_4}\n${e.row_5}\n${e.row_6}\n${e.row_7}`
    return final_data
}
console.log(CAL(5,2045,' ')) // month: 5 year: 2045