use lexer::input::Input;

static _lexer_actions: [i8 ; 9] = [ 0, 1, 0, 1, 1, 1, 2, 0 , 0 ];
static _lexer_key_offsets: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_trans_keys: [u8 ; 2] = [ 0 , 0 ];
static _lexer_single_lengths: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_range_lengths: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_index_offsets: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_trans_cond_spaces: [i8 ; 3] = [ -1, 0 , 0 ];
static _lexer_trans_offsets: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_trans_lengths: [i8 ; 3] = [ 1, 0 , 0 ];
static _lexer_cond_keys: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_cond_targs: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_cond_actions: [i8 ; 3] = [ 5, 0 , 0 ];
static _lexer_to_state_actions: [i8 ; 3] = [ 1, 0 , 0 ];
static _lexer_from_state_actions: [i8 ; 3] = [ 3, 0 , 0 ];
static _lexer_eof_cond_spaces: [i8 ; 3] = [ -1, 0 , 0 ];
static _lexer_eof_cond_key_offs: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_eof_cond_key_lens: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_eof_cond_keys: [i8 ; 2] = [ 0 , 0 ];
static _lexer_nfa_targs: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_nfa_offsets: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_nfa_push_actions: [i8 ; 3] = [ 0, 0 , 0 ];
static _lexer_nfa_pop_trans: [i8 ; 3] = [ 0, 0 , 0 ];
static lexer_start : i32 = 0;
static lexer_error : i32 = -1;
static lexer_en_line_begin : i32 = 0;
pub struct Lexer {
	input: Input,
	
	// for ragel
	cs: i32,
	p: i32,
	pe: i32,
	ts: i32,
	te: i32,
}

impl Lexer {
	pub fn new(input: String) -> Lexer {
		let input = Input::new(input);
		
		let cs;
		let ts;
		let te;
		
		
		{
			cs = ( lexer_start ) as i32;
			ts = 0;
			te = 0;
		}
		Lexer {
			input,
			cs, ts, te,
			p: 0,
			pe: 0
		}
	}
	
	pub fn advance(&mut self) {
		let data = self.input.clone();
		
		// TODO macro
		let mut cs = self.cs;
		let mut p = self.p;
		let mut pe = self.pe;
		let mut ts = self.ts;
		let mut te = self.te;
		
		
		{
			let mut _trans  = 0;
			let mut _have  = 0;
			let mut _cont  = 1;
			let mut _klen = 0;
			let mut _keys :i32= 0;
			let mut _acts :i32= 0;
			let mut _nacts = 0;
			while ( _cont == 1  )
			{
			
				_have = 0;
				if ( p == pe  ) {
					{
						if ( _have == 0  ) {
							_cont = 0;
							
						}
					}
					
				}
				if ( _cont == 1  ) {
					{
						if ( _have == 0  ) {
							{
								_acts = ( _lexer_from_state_actions[(cs) as usize] ) as i32;
								_nacts = ( _lexer_actions[(_acts ) as usize]
								) as u32;
								_acts += 1;
								while ( _nacts > 0  )
								{
									match ( _lexer_actions[(_acts ) as usize]
									) {
										1  => {
											{{ts = p;
												}}
											
										}
										
										_ => {}
									}
									_nacts -= 1;
									_acts += 1;
								}
								
								
								_keys = ( _lexer_key_offsets[(cs) as usize] ) as i32;
								_trans = ( _lexer_index_offsets[(cs) as usize] ) as u32;
								_have = 0;
								_klen = ( _lexer_single_lengths[(cs) as usize] ) as i32;
								if ( _klen > 0  ) {
									{
										let mut _lower  :i32= _keys;
										let mut _upper  :i32= _keys + _klen - 1;
										let mut _mid :i32= 0;
										while ( _upper >= _lower && _have == 0  )
										{
											_mid = _lower + ((_upper-_lower) >> 1);
											if ( ( data[(p ) as usize]
											) < _lexer_trans_keys[(_mid ) as usize]
											) {
												_upper = _mid - 1;
												
											}
											else if ( ( data[(p ) as usize]
											) > _lexer_trans_keys[(_mid ) as usize]
											) {
												_lower = _mid + 1;
												
											}
											else {
												{
													_trans += ( (_mid - _keys) ) as u32;
													_have = 1;
												}
												
											}
										}
										
										if ( _have == 0  ) {
											{
												_keys += _klen;
												_trans += ( _klen ) as u32;
											}
											
										}
									}
									
									
								}
								if ( _have == 0  ) {
									{
										_klen = ( _lexer_range_lengths[(cs) as usize] ) as i32;
										if ( _klen > 0  ) {
											{
												let mut _lower  :i32= _keys;
												let mut _mid :i32= 0;
												let mut _upper  :i32= _keys + (_klen<<1) - 2;
												while ( _have == 0 && _lower <= _upper  )
												{
													_mid = _lower + (((_upper-_lower) >> 1) & !1
													);
													if ( ( data[(p ) as usize]
													) < _lexer_trans_keys[(_mid ) as usize]
													) {
														_upper = _mid - 2;
														
													}
													else if ( ( data[(p ) as usize]
													) > _lexer_trans_keys[(_mid + 1 ) as usize]
													) {
														_lower = _mid + 2;
														
													}
													else {
														{
															_trans += ( ((_mid - _keys)>>1) ) as u32;
															_have = 1;
														}
														
													}
												}
												
												if ( _have == 0  ) {
													_trans += ( _klen ) as u32;
													
												}
											}
											
										}
									}
									
									
								}
							}
							
						}
						if ( _cont == 1  ) {
							{
								cs = ( _lexer_cond_targs[(_trans) as usize] ) as i32;
								if ( _lexer_cond_actions[(_trans) as usize]!= 0  ) {
									{
										_acts = ( _lexer_cond_actions[(_trans) as usize] ) as i32;
										_nacts = ( _lexer_actions[(_acts ) as usize]
										) as u32;
										_acts += 1;
										while ( _nacts > 0  )
										{
											match ( _lexer_actions[(_acts ) as usize]
											) {
												2  => {
													{{te = p+1;
														}}
													
												}
												
												_ => {}
											}
											_nacts -= 1;
											_acts += 1;
										}
										
										
									}
									
								}
								_acts = ( _lexer_to_state_actions[(cs) as usize] ) as i32;
								_nacts = ( _lexer_actions[(_acts ) as usize]
								) as u32;
								_acts += 1;
								while ( _nacts > 0  )
								{
									match ( _lexer_actions[(_acts ) as usize]
									) {
										0  => {
											{{ts = 0;
												}}
											
										}
										
										_ => {}
									}
									_nacts -= 1;
									_acts += 1;
								}
								
								
								if ( _cont == 1  ) {
									p += 1;
									
								}
							}
							
						}
					}
					
				}
			}
			
		}
		self.cs = cs;
		self.p = p;
		self.pe = pe;
		self.ts = ts;
		self.te = te;
	}
}
