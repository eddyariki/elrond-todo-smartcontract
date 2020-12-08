use elrond_wasm::elrond_codec::*;
use elrond_wasm::{ Vec};


pub struct Task{
    pub content: Vec<u8>,
    pub completed: bool
}

impl NestedEncode for Task{
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.content.dep_encode(dest)?;
		self.completed.dep_encode(dest)?;
		Ok(())
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.content.dep_encode_or_exit(dest, c.clone(), exit);
		self.completed.dep_encode_or_exit(dest, c.clone(), exit);
		
	}
}

impl TopEncode for Task {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		top_encode_from_nested(self, output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		top_encode_from_nested_or_exit(self, output, c, exit);
	}
}

impl NestedDecode for Task {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Task {
			content: Vec::<u8>::dep_decode(input)?,
			completed: bool::dep_decode(input)?,
		})
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Task {
			content: Vec::<u8>::dep_decode_or_exit(input, c.clone(), exit),
			completed: bool::dep_decode_or_exit(input, c.clone(), exit),
			
		}
	}
}

impl TopDecode for Task {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		top_decode_from_nested_or_exit(input, c, exit)
	}
}