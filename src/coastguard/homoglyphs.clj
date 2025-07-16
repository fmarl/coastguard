(ns coastguard.homoglyphs)

(def homoglyphs
  {\A [\А \Ꭺ \Α \ᴀ \ꓮ \Ɐ ]
   \B [\Β \В \Ᏼ \ʙ \Ꞗ ]
   \C [\Ϲ \С \Ꮯ \Ⅽ \Ⲥ ]
   \D [\Ꭰ \Ⅾ \ԁ \ꓓ ]
   \E [\Ε \Е \Ꭼ \ⅇ \ꓰ ]
   \F [\Ϝ \Ғ \ᖴ ]
   \G [\Ԍ \ɢ \Ɠ ]
   \H [\Η \Н \Ꮋ \ʜ \Ⲏ ]
   \I [\Ι \І \Ӏ \Ꭵ \ꓲ ]
   \J [\Ј \Ϳ \ʝ ]
   \K [\Κ \К \Ꮶ \Ⲕ ]
   \L [\Ꮮ \ʟ \Ⳑ ]
   \M [\Μ \М \Ꮇ \Ⅿ \Ⲙ ]
   \N [\Ν \Ⲛ \Ꮋ \ꓠ ]
   \O [\Ο \О \Օ \Ⲟ \ꓳ ]
   \P [\Ρ \Р \Ꮲ \Ⲣ ]
   \Q [\Ⴓ ]
   \R [\Ꮢ \ʀ \Ⲣ ]
   \S [\Ѕ \Տ \Ꮪ \ꓢ ]
   \T [\Τ \Т \Ꭲ \Ⲧ ]
   \U [\Ս \Ս \Ս \⋃ \ꓴ ]
   \V [\Ѵ \Ꮩ \ⴸ ]
   \W [\Ԝ \Ꮃ ]
   \X [\Χ \Х \Ⅹ \Ⲭ ]
   \Y [\Υ \Ү \Ⲩ \ʏ ]
   \Z [\Ζ \Ꮓ \Ⲍ ]
   \a [\а \ɑ \ᴀ ]
   \b [\Ь \Ꮟ \Ƅ ]
   \c [\ϲ \с \ⅽ ]
   \d [\ԁ \ɗ ]
   \e [\е \ҽ ]
   \f [\ϝ \ғ ]
   \g [\ɡ \ɢ ]
   \h [\һ \հ ]
   \i [\і \Ꭵ \ı ]
   \j [\ј ]
   \k [\κ \к ]
   \l [\ⅼ \ӏ \Ɩ ]
   \m [\м ]
   \n [\п \ո ]
   \o [\ο \о \օ ]
   \p [\ρ \р ]
   \q [\զ ]
   \r [\г \ᴦ ]
   \s [\ѕ ]
   \t [\τ \т ]
   \u [\υ \ս ]
   \v [\ѵ ]
   \w [\ѡ ]
   \x [\х ]
   \y [\у \ү ]
   \z [\ᴢ \ȥ ]})

(defn get-homoglyphs [char]
  (concat (get homoglyphs char) [char]))

(defn mutate-with-homoglyphs [name]
  (map get-homoglyphs (seq (chars (char-array name)))))

(defn mutations-into-name [mutations]
    (if (empty? mutations)
     [""]
     (for [head (first mutations)
           tail (mutations-into-name (rest mutations))]
       (str head tail))))

(defn mutate-using-homoglyphs [name]
  (let [mutations (mutate-with-homoglyphs name)]
    (mutations-into-name mutations)))