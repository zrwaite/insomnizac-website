class CreateUsers < ActiveRecord::Migration[7.0]
  def change
    create_table :users do |t|
      t.string :email
      t.string :password_hash
      t.boolean :confirmed

      t.timestamps
    end
  end
end
