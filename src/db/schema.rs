table! {
	/// Representation of the `saved_rolls` table.
	///
	/// (Automatically generated by Diesel.)
	saved_rolls (guild_id, user_id, name) {
		/// The `guild_id` column of the `saved_rolls` table.
		///
		/// Its SQL type is `BigInt`.
		///
		/// (Automatically generated by Diesel.)
		guild_id -> BigInt,
		/// The `user_id` column of the `saved_rolls` table.
		///
		/// Its SQL type is `BigInt`.
		///
		/// (Automatically generated by Diesel.)
		user_id -> BigInt,
		/// The `name` column of the `saved_rolls` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		name -> Text,
		/// The `command` column of the `saved_rolls` table.
		///
		/// Its SQL type is `Text`.
		///
		/// (Automatically generated by Diesel.)
		command -> Text,
	}
}
