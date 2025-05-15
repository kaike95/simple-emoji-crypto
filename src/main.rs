use std::io::Write; // permite acesso ao stdout().flush()

fn getinput(text: &str) -> String {
    // função utilitária: obtém input em string
    print!("{text}");
    std::io::stdout().flush().unwrap();
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let input = buffer.trim().to_uppercase();
    input
}

fn mult_matriz2(m1: Vec<i32>, m2: Vec<i32>) -> Vec<i32> {
    // função utilitária: multiplica dois vetores (matrizes)
    let mut mf: Vec<i32> = Vec::new();
    mf.push((m1[0] * m2[0]) + (m1[1] * m2[2])); //1,1
    mf.push((m1[0] * m2[1]) + (m1[1] * m2[3])); //2,1
    mf.push((m1[2] * m2[0]) + (m1[3] * m2[2])); //1,2
    mf.push((m1[2] * m2[1]) + (m1[3] * m2[3])); //2,2
    return mf;
}

fn inv_matriz2(matriz: Vec<i32>) -> Vec<i32> {
    // função utilitária: matriz inversa
    let mut mi: Vec<i32> = Vec::new();
    mi.push(matriz[3]); //1,1
    mi.push(matriz[1] - (2 * matriz[1])); //2,1
    mi.push(matriz[2] - (2 * matriz[2])); //1,2
    mi.push(matriz[0]); //2,2
    return mi;
}

fn cripto_palavra(input: &String) -> Vec<i32> {
    // função base: transforma 4 letras em uma matriz 2x2
    let input = input.replace(" ", "@"); // troca espaços por @
    let letras: Vec<char> = vec![
        '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut criptografada: Vec<i32> = Vec::new();
    let mut iterinput = input.chars().into_iter();
    while let Some(i) = iterinput.next() {
        criptografada.push(letras.iter().position(|&idx| idx == i).unwrap() as i32)
        // acha a posição da letra e coloca no vetor (IndexOf no C#)
    }
    while criptografada.len() % 4 != 0 {
        // completa a matriz 2x2 com espaços (@) caso não há suficientes numeros
        criptografada.push(0);
    }
    return criptografada;
}

fn emojificar(input: String, modo: bool) -> String {
    // função base: possui modo de decodificar emojis e transformar em emojis
    let mut output: String = String::new();
    let emojis: Vec<char> = vec![
        '💤', '💣', '🚛', '🔲', '♑', '🍻', '💼', '😾', '🚘', '👮', '📛', '🚀', '😜', '💜', '🗿',
        '🚜', '🗃', '🏬', '🚻', '🌜', '🐙', '🌮', '🎥', '〽', '🎫', '🐑', '🏦', '🍘', '🐥', '🐲',
        '🕷', '👌', '😛', '🔊', '📨', '🌂', '🖕', '🍪', '➖', '💯', '🍺', '📰', '❓', '🚿', '🎣',
        '📌', '🐠', '🌱', '📧', '🈶', '🕶', '🐱', '⏭', '🏤', '😍', '↙', '♐', '🏉', '📮', '🎢',
        '🚚', '😭', '🈁', '🎄', '🚼', '📺', '😫', '🌝', '👏', '👼', '🏹', '😪', '🔢', '🎓', '🕯',
        '🎉', '🏘', '🐁', '🌍', '🍁', '🏌', '💾',
    ]; // 81 emojis, para possibilitar 3 criptografias diferentes, dependendo de condições
       // pode-se customizar os emojis, caso queira, ou aumentar a quantia até 120

    let mut emojisel: Vec<char> = Vec::new();

    if modo == true {
        // transformar em emoji
        let mut offset = input.len() % 3; // obtém a sequencia de emojis para serem usados
        for _ in 0..=26 {
            emojisel.push(emojis[offset]); // cria a selecão dependendo do acima
            offset += 3
        }
        for x in 0..=2 {
            output += emojisel[x as usize].to_string().as_str(); // adiciona a primeira bandeira
        }

        let pronta = cripto_palavra(&input); // transforma a palavra em indíces para virarem emojis
        for x in pronta {
            output += emojisel[x as usize].to_string().as_str();
        }
    } else {
        // transformar emoji em letras
        let mut input = input.chars();
        let mut offset: usize;
        let mut three: String = String::new();
        let findthree: Vec<char> = input.clone().into_iter().collect(); // encontra a bandeira
        for x in 0..=2 {
            three += findthree[x].to_string().as_str();
            input.next();
        }
        match three.as_str() {
            // encontra a sequencia para decodificar
            "💤🔲💼" => offset = 0,
            "💣♑😾" => offset = 1,
            _ => offset = 2,
        }
        for _ in 0..=26 {
            emojisel.push(emojis[offset]); // recria a sequencia
            offset += 3
        }

        let mut idxs: Vec<i32> = Vec::new();
        while let Some(i) = input.next() {
            // traduz para índices para virar letras
            idxs.push(emojisel.iter().position(|&idx| idx == i).unwrap() as i32)
        }
        output = descripto_palavra(idxs)
    }
    return output;
}

fn descripto_palavra(input: Vec<i32>) -> String {
    // função base: transforma índices em letras
    let letras: Vec<char> = vec![
        '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut palavrafinal: String = String::new();
    let mut iterinput = input.iter();
    while let Some(n) = iterinput.next() {
        let mut idx = *n;
        if idx > 26 {
            idx = idx % 26
        } else if idx < 0 {
            while idx < 0 {
                idx += 26;
            }
        }
        let idx: usize = idx.try_into().unwrap();
        palavrafinal += letras[idx].to_string().as_str();
    }
    return palavrafinal.replace("@", " ");
}

fn determinante(matriz: &Vec<i32>) -> i32 {
    // função utilitária: calcula determinante 2x2
    let det = (matriz[0] * matriz[3]) - (matriz[1] * matriz[2]);
    return det;
}

fn gerar_chave(input: String) -> Vec<i32> {
    // função base: gera a chave de acordo com a frase dada
    let iterinput: Vec<i32> = cripto_palavra(&input);
    if determinante(&iterinput).abs() != 1 {
        // se determinante não for |1|, adapte a chave
        let finalnum: i32 = (iterinput[0] * iterinput[3]) - 1;
        let chave: Vec<i32> = vec![iterinput[0], finalnum, 1, iterinput[3]];
        return chave;
    } else {
        return iterinput;
    }
}

fn criptografar(mut input: String, chave: Vec<i32>) -> String {
    // função base: chave * matriz descriptografada
    let mut palavra: String = String::new();
    while input.len() % 4 != 0 {
        // garante que a matriz esteja cheia ao adicionar espaços (0, @)
        input += " ";
    }
    let rep = (input.len() as f32 / 4.0).ceil() as usize;
    let mut idx: usize = 0;
    for _ in 1..=rep {
        // vezes que a frase foi dividida em 2x2
        let matriz: Vec<i32> = mult_matriz2(
            chave.clone(),
            cripto_palavra(&input[idx..=(idx + 3)].to_string()),
        );
        idx += 4;
        palavra += descripto_palavra(matriz).as_str() // soma as partes
    }
    return emojificar(palavra, true);
}

fn descriptografar(input: String, chave: String) -> String {
    // função base: inversa chave * matriz criptografada
    let mut palavra: String = String::new();
    let input = emojificar(input, false); // descodifica de emojis para numeros
    let rep = (input.len() as f32 / 4.0).ceil() as usize;
    let mut idx: usize = 0;
    for _ in 1..=rep {
        let matriz: Vec<i32> = mult_matriz2(
            // retorna à matriz original
            inv_matriz2(gerar_chave(chave.clone())),
            cripto_palavra(&input[idx..=(idx + 3)].to_string()),
        );
        idx += 4;
        palavra += descripto_palavra(matriz).as_str();
    }
    return palavra;
}

fn main() {
    let input: String;
    let opc = getinput(
        "O que você quer fazer?\n1 - Criptografar\n2 - Descriptografar (necessita-se de uma chave)\nSua opção: ",
    );

    match opc.parse::<u8>().unwrap() {
        1 => {
            input = getinput("Digite sua frase para ser criptografada: ");
            let chave: Vec<i32> = gerar_chave(getinput("Digite sua senha (apenas letras): "));
            println!("{}", criptografar(input, chave));
        }
        2 => {
            input = getinput("Digite sua frase para ser descriptografada: ");
            let chave = getinput("Digite a sua senha: ");
            println!("{}", descriptografar(input, chave));
        }
        _ => {
            println!("Opção inválida");
        }
    };
}
