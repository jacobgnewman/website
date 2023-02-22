
use std::borrow::Cow;
// latex parsing
use katex::{render_with_opts, Opts};

use regex::{Captures, Regex};
// markdown parsing
//use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};


fn parse_latex(text: String) -> Option<String> {
    // options for latex renderer
    let opts = Opts::builder()
        .output_type(katex::opts::OutputType::Mathml)
        .build()
        .unwrap();

    // regex to match on $$<foo>$$ or $<foo>$
    let re = Regex::new(r"\$[^$]+\$|\$\$[^$]+\$\$").unwrap();
    let tex: &str = &*text;
    let result: Cow<str> = re.replace_all(tex, |caps: &Captures| {
        let expression = &caps[0];
        let expression = expression.trim_matches('$');
        let post = render_with_opts(expression, &opts).unwrap();
        post
    });
    let result = result.to_string();
    Some(result)
}

pub fn md_to_html(text: String) ->Option<String> {
    let contents = parse_latex(text).unwrap();
    let arena = Arena::new();

    let mut options = &mut ComrakOptions::default();

    // set front matter delim
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.tagfilter = true;
    options.render.unsafe_ = true;

    let root = parse_document(&arena, &contents, options);

    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();

    Some(String::from_utf8(html).unwrap())
}

    //fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    //where
        //F: Fn(&'a AstNode<'a>),
    //{
        //f(node);
        //for n in node.children() {
            //iter_nodes(n, f);
        //}
    //}
    //iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        //NodeValue::FrontMatter(ref mut block) => {
            //println!(
                //"Frontmatter: {}",
                //String::from_utf8(block.to_vec()).unwrap()
            //);
        //}
        //_ => (),
    //});