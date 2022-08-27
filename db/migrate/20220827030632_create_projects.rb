class CreateProjects < ActiveRecord::Migration[7.0]
  def change
    create_table :projects, id: false do |t|
      t.string :name, null: false
      t.string :description
      t.string :github_name

      t.timestamps
    end
    execute "ALTER TABLE projects ADD PRIMARY KEY (name);"
  end
end
