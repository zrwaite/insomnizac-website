class AddDefaultValuesToUsers < ActiveRecord::Migration[7.0]
  def change
    change_column :users, :confirmed, :boolean, :default => false
  end
end
