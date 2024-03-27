use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

/// insert data to template
#[derive(Parser)]
#[clap(
    name="insert_data",
    author="masamichi",
    version="v0.0.1",
    about="insert data to template"
)]
struct Args {
    /// template
    #[arg(short, long)]
    template: String,

    ///  input data
    #[arg(short, long)]
    input_data: String,

    /// chunk of row
    #[arg(short, long)]
    row: u8,

    /// between space
    #[arg( long, default_value_t=1)]
    space: u8,


    
}

fn main()-> Result<(), Box<dyn std::error::Error>> {

    let arg: Args = Args::parse();

    // open template
    let mut template = File::open(arg.template)?;
    let mut template_contents = String::new();
    template.read_to_string(&mut template_contents)?;

    // open input data
    let mut input = File::open(arg.input_data)?;
    let mut input_contents = String::new();
    input.read_to_string(&mut input_contents)?;

    // input dataを配列に変換

    let chunk=arg.row;
    // 文字列を改行で分割して配列に
    let input_array:Vec<&str> = input_contents.split("\n").collect();

    // 結果の入れ物
    let mut buffer:Vec<String> = Vec::new();



    let mut template_buffer:String = template_contents.clone();

    for (i, val) in input_array.into_iter().enumerate(){

        // プレイスホルダーを定義　１～ chunk
        // データの行数をスペース＋Chunkで割ることで、あまりを取得
        // プレースホルダを置き換えていく
        let number:usize = (i+1) % usize::from(chunk+arg.space);
        let placeholder = format!("${}", number);

        let removed:String = val.replace("\r", "");

        template_buffer = template_buffer.replace(&placeholder , &removed);

        // あまりがchunkになったらそれは次の塊に移るとき
        // バッファーに書き込んで次のデータへ移行
        if number == usize::from(chunk){
            let temp_clone = template_buffer.clone();
            buffer.push(temp_clone);
            template_buffer = template_contents.clone();

        }
    }

    // バッファーをファイルへ書き込む
    let mut output = File::create("output.txt")?;
    let s:String = buffer.join("\n");
    
    write!(output, "{}", s)?;
    output.flush()?;



    Ok(())

}
