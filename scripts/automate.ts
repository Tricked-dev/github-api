#!/bin/deno
const fmt = Deno.run({
  cmd: ["deno", "fmt"],
  stderr: "inherit",
  stdout: "inherit",
}).status();
import api from "./api.json" assert { type: "json" };

const data = [];
const enums = [];
const getMethod = [];
const getPath = [];
const implementsFunctions = [];
const structs = [];
const functions = [];
const b: Record<any, any> = {};
const methods = new Set();
const typemap: Record<string, string> = {
  integer: "i64",
  number: "f64",
  string: "String",
  boolean: "bool",
  array: "Vec",
  object: "Value",
};

function generateDocString(
  tags: string,
  method: string,
  path: string,
  docUrl: string,
  summary: string,
  description: string,
) {
  return [
    `* tags ${tags}`,
    `* ${method} \`${path}\``,
    `${docUrl ? `* docs <${docUrl}>` : ""}\n`,
    `${summary}`,
    `${description}`,
  ].join("\n");
}
function generateFunction(
  docs: string,
  name: string,
  args: string,
  response: string,
  vars: string,
) {
  return `${docs}\npub async fn ${name}<T, V>(&self,${args}, query: Option<&T>, body: Option<V>) -> Result<${response}Response,Error> where
        T: Serialize + ?Sized,
        V: Into<Body>, { self.req(EndPoints::${response}(${vars}), query, body).await  }`
    .replace(
      /,,/g,
      ",",
    );
}

function upperFirst(i: string): string {
  return i.charAt(0).toUpperCase() + i.slice(1);
}

function iter(name: any, obj: any, types: Record<string, any>) {
  if (obj.type) {
    let option = false;
    if (Array.isArray(obj.type)) {
      let arr = new Set<string>(obj.type);
      if (arr.size == 2 && arr.has("null")) {
        option = true;
        arr.delete("null");
        obj.type = `${Array.from(arr).join("")}`;
        // b[obj.type] = Array.from(arr).join(",\n")

        typemap[obj.type] = `${
          Array.from(arr)
            .map((x) => typemap[x])
            .join("")
        }`;
      } else {
        // console.log(obj.type)
        let types = obj.type.map((x: string | number) => typemap[x]);
        // console.log(types)
        let name = types.map(upperFirst).join("");
        b[name] = types.join(",\n");
        typemap[obj.type] = name;
        obj.type = name;
      }
    }
    // if (!typemap[obj.type]) console.log(obj.type, typemap[obj.type],typemap);
    const defaults = { doc: obj.description, example: obj.example };
    if (obj.type == "array") {
      if (obj.items.type) {
        // console.log(typemap[obj.items.type])
        if (option) {
          types[name] = {
            type: `Option<Vec<${typemap[obj.items.type]}>>`,
            ...defaults,
          };
        } else {
          types[name] = {
            type: `Vec<${typemap[obj.items.type]}>`,
            ...defaults,
          };
        }
      }
    } else {
      if (option) {
        types[name] = {
          type: `Option<${typemap[obj.type]}>`,
          ...defaults,
        };
      } else {
        types[name] = {
          type: typemap[obj.type],
          ...defaults,
        };
      }
    }
  } else {
    if (
      obj?.anyOf?.length == 2 &&
      obj?.anyOf.find((x: any) => x.type === "null")
    ) {
      let response = obj.anyOf.find((x: any) => x.type !== "null")!;
      for (const [name, obj] of Object.entries(response) as any) {
        iter(name, obj, types);
      }
    }
  }
}

for (const [key, val] of Object.entries(api.paths)) {
  for (const [method, values] of Object.entries(val as any) as any) {
    methods.add(upperFirst(method));
    const vars: string[] = [];
    key.replace(/{(.*?)}/g, (_, g1, _1) => {
      vars.push(g1.replace("ref", "aref"));
      return "";
    });
    const tried = (num: number | string) =>
      values.responses?.[num.toString()]?.content?.["application/json"]?.schema
        .properties;

    const response = tried(200) ||
      tried(201) ||
      tried(204) ||
      tried(202) ||
      tried(203) ||
      tried(205) ||
      tried(206) ||
      tried(207) ||
      tried(208) ||
      tried(209) ||
      tried(2010) ||
      tried(2011);
    const types: Record<string, any> = {};

    if (response) {
      for (const [name, obj] of Object.entries(response) as any) {
        iter(name, obj, types);
      }
    }
    data.push({
      name: upperFirst(method) +
        key
          .split(/\/|\-|_/gim)
          .map((x: string) => upperFirst(x))
          .join("")
          .replace(/{|}/gim, ""),
      dosc: values.description,
      path: key
        .replace("{ref}", "{aref}")
        .replace("{content_reference_id}", "{content_areference_id}"),
      method: method,
      vars: vars,
      docs: generateDocString(
        values.tags.join(", "),
        method,
        key,
        values?.externalDocs?.url,
        values.summary,
        values.description,
      ),
      types,
    });
  }
}

let toWrite = [
  `use serde::{Deserialize, Serialize};\n` + `use serde_json::Value;\n`,
  // `use std::collections::HashMap;\n`,
  `pub enum Methods { \n    ${Array.from(methods).join(",\n    ")} \n}`,
];

for (const key of data) {
  enums.push(
    key.docs
      .split("\n")
      .map((x) => "	/// " + x)
      .join("\n") +
      `\n${key.name}(${"String,".repeat(key.vars.length).slice(0, -1)})`,
  );
  if (Object.entries(key.types).length !== 0) {
    const items = [];
    // const methods = []
    for (let [keys, val] of Object.entries(key.types)) {
      if (val.type == undefined) continue;
      const docs = `${
        val.example
          ? `* example - ${val.example}`
          : "" + `${val.doc ? `\n${val.doc}` : ""}`
      }`
        .trim()
        .split("\n")
        .map((x) => `	/// ${x}`)
        .join("\n");
      if (keys === "type" || keys === "ref") {
        items.push(
          `${docs}\n#[serde(rename = "${keys}")]\na${keys}: ${val.type}\n`,
        );
        keys = "a" + keys;
      } else {
        items.push(`${docs}\n${keys}: ${val.type}`);
      }

      // methods.push(`pub fn ${keys}(&self) -> ${val.type} { self.${keys} }`)
    }

    //\nimpl ${key.name}Response {\n${methods.join("\n\n")}\n  }
    functions.push(
      generateFunction(
        key.docs
          .split("\n")
          .map((x) => "	/// " + x)
          .join("\n"),
        key.method +
          key.path
            .split(/\/|\-|_/gim)
            .map((x: string) => x)
            .join("_")
            .replace(/{|}/gim, ""),
        key.vars.map((x) => `${x}:String`).join(", "),
        key.name,
        key.vars.join(", "),
      ),
    );
  }

  getMethod.push([key.name, upperFirst(key.method)]);
  getPath.push([key.name, key.path, key.vars]);
}

implementsFunctions.push(
  `pub const fn method(&self) -> Methods { match *self { ${
    getMethod
      .map((x) => `EndPoints::${x[0]}(..) => Methods::${x[1]}`)
      .join(",\n  ")
  }} }`,
  `pub fn path(&self) -> String { match self { ${
    getPath
      .map(
        (x) =>
          //@ts-ignore -
          `EndPoints::${x[0]}(${x[2].join(",")}) => format!("${x[1]}", ${
            x[2]
              //@ts-ignore -
              .map((x) => `${x} = ${x}`)
              .join(",")
          })`,
      )
      .join(",\n  ")
  }} }`,
);

for (const [key, val] of Object.entries(b)) {
  structs.push(`pub enum ${key} { \n${val}\n}`);
}

toWrite.push(`pub enum EndPoints { \n    ${enums.join(",\n    ")} \n}`);
toWrite.push(`impl EndPoints {  ${implementsFunctions.join("\n")} }`);
toWrite.push(structs.join("\n"));

Deno.writeTextFileSync("src/end_points.rs", toWrite.join("\n\n"));
// Deno.writeTextFileSync(
//   "src/implements.rs",
//   `use crate::{client::Client, end_points::*, Error};\nuse reqwest::Body;\nuse serde::{Deserialize, Serialize};\nimpl Client {\n${
//     functions.join(
//       "\n",
//     )
//   } \n}`,
// );
console.log("Running cleanup commands.");
await Promise.all([
  fmt,

  Deno.run({
    cmd: ["cargo", "clippy", "--fix", "--allow-dirty"],
    stderr: "inherit",
    stdout: "inherit",
  }).status(),
]);
await Deno.run({
  cmd: ["cargo", "fmt"],
  stderr: "inherit",
  stdout: "inherit",
}).status();
