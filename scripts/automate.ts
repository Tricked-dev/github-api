#!/bin/deno
const api = JSON.parse(Deno.readTextFileSync('scripts/api.json'));

const data = [];

const methods = new Set();

function upperFirst(i: string): string {
	return i.charAt(0).toUpperCase() + i.slice(1);
}

// let enums = [];

for (const [key, val] of Object.entries(api.paths)) {
	for (const [method, values] of Object.entries(val as any) as any) {
		methods.add(upperFirst(method));
		const vars: string[] = [];
		key.replace(/{(.*?)}/g, (_, g1, _1) => {
			vars.push(g1.replace('ref', 'aref'));
			return '';
		});
		let response =
			values.responses?.['200']?.content?.['application/json']?.schema
				.properties;
		let types: Record<string, any> = {};
		// console.log(response);
		// console.log(types);
		let typemap: Record<string, string> = {
			integer: 'i64',
			number: 'f64',
			string: 'String',
			boolean: 'bool',
			array: 'Vec',
			object: 'Value',
		};
		if (response)
			for (const [name, obj] of Object.entries(response) as any) {
				if (obj.type) {
					if (!typemap[obj.type]) console.log(obj.type, typemap[obj.type]);
					let defaults = { doc: obj.description, example: obj.example };
					if (obj.type == 'array') {
						if (obj.items.type)
							types[name] = {
								type: `Vec<${typemap[obj.items.type]}>`,
								...defaults,
							};
					} else {
						types[name] = {
							type: typemap[obj.type],
							...defaults,
						};
					}
				}
			}
		// console.log(types);
		data.push({
			name:
				upperFirst(method) +
				key
					.split(/\/|\-|_/gim)
					.map((x: string) => upperFirst(x))
					.join('')
					.replace(/{|}/gim, ''),
			dosc: values.description,
			path: key
				.replace('{ref}', '{aref}')
				.replace('{content_reference_id}', '{content_areference_id}'),
			method: method,
			vars: vars,
			docs:
				`* tags ${values.tags.join(', ')}\n` +
				`* ${method} \`${key}\`\n` +
				`${
					values?.externalDocs?.url ? `* docs ${values?.externalDocs?.url}` : ''
				}\n\n` +
				`${values.summary}\n` +
				`${values.description}`,
			types,
		});
	}
}
let toWrite = [
	`use serde::{Deserialize, Serialize};\n` +
		`use serde_json::Value;\n` +
		`use std::collections::HashMap;\n` +
		``,
	`pub enum Methods { \n    ${Array.from(methods).join(',\n    ')} \n}`,
];
let enums = [];
const getMethod = [];
const getPath = [];
const implementsFunctions = [];
const structs = [];
const functions = [];
for (const key of data) {
	enums.push(
		key.docs
			.split('\n')
			.map((x) => '	/// ' + x)
			.join('\n') +
			`\n${key.name}(${'String,'.repeat(key.vars.length).slice(0, -1)})`
	);
	if (Object.entries(key.types).length !== 0) {
		let items = [];
		for (const [keys, val] of Object.entries(key.types)) {
			let docs = `${
				val.example
					? `* example - ${val.example}`
					: '' + `${val.doc ? `\n${val.doc}` : ''}`
			}`
				.trim()
				.split('\n')
				.map((x) => `	/// ${x}`)
				.join('\n');
			if (keys === 'type' || keys === 'ref') {
				items.push(
					`${docs}\n#[serde(rename = "${keys}")]\na${keys}: ${val.type}\n`
				);
			} else {
				items.push(`${docs}\n${keys}: ${val.type}`);
			}
		}
		structs.push(
			`#[derive(Serialize, Deserialize, Clone, Debug)]\npub struct ${
				key.name
			}Response {\n ${items.join(',\n	')} \n}`
		);
		functions.push(
			`pub async fn ${
				key.method +
				key.path
					.split(/\/|\-|_/gim)
					.map((x: string) => x)
					.join('_')
					.replace(/{|}/gim, '')
			}<T, V>(&self,${key.vars
				.map((x) => `${x}:String`)
				.join(', ')}, query: Option<&T>, body: Option<V>) -> Result<${
				key.name
			}Response,Error> where
        T: Serialize + ?Sized,
        V: Into<Body>, { self.req(EndPoints::${key.name}(${key.vars.join(
				', '
			)}), query, body).await  }`.replace(',,', ',')
		);
	}

	getMethod.push([key.name, upperFirst(key.method)]);
	getPath.push([key.name, key.path, key.vars]);
}

implementsFunctions.push(
	`pub const fn method(&self) -> Methods { match *self { ${getMethod
		.map((x) => `EndPoints::${x[0]}(..) => Methods::${x[1]}`)
		.join(',\n  ')}} }`,
	`pub fn path(&self) -> String { match self { ${getPath
		.map(
			(x) =>
				//@ts-ignore -
				`EndPoints::${x[0]}(${x[2].join(',')}) => format!("${x[1]}",${x[2]
					//@ts-ignore -
					.map((x) => `${x} = ${x}`)
					.join(',')} )`
		)
		.join(',\n  ')}} }`
);

toWrite.push(`pub enum EndPoints { \n    ${enums.join(',\n    ')} \n}`);
toWrite.push(`impl EndPoints {  ${implementsFunctions.join('\n')} }`);
toWrite.push(structs.join('\n'));
Deno.writeTextFileSync(
	'src/implements.rs',
	`use crate::{client::Client, end_points::*, Error};\nuse reqwest::Body;\nuse serde::{Deserialize, Serialize};\nimpl Client {\n${functions.join(
		'\n'
	)} \n}`
);
Deno.writeTextFileSync('src/end_points.rs', toWrite.join('\n\n'));
// console.log(`enum EndPoints { ${enums.join(',\n')} }`);
// console.log(data);
