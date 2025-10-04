# 時計絵文字をUnicodeコードポイントで生成して出力
# 🕐 (U+1F550) 〜 🕛 (U+1F55B)
# 🕧 (U+1F567) 〜 🕦 (U+1F566) は半の絵文字

def clock_emojis():
    clocks = []
    # 0時〜11時の「○時」と「○時半」を順に追加
    for hour in range(12):
        # ○時
        full_hour = chr(0x1F550 + (hour % 12))
        clocks.append(full_hour)
        # ○時半
        half_hour = chr(0x1F55C + (hour % 12))
        clocks.append(half_hour)
    return clocks

for emoji, (h, m) in zip(clock_emojis(), [(h//2, (h%2)*30) for h in range(24)]):

    h = (h + 1) % 12
    print(f"if h == {h} && {0 + m} <= m && m < {30 + m} {{ \"{emoji}\".to_string() }}")

