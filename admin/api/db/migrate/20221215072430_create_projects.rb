class CreateProjects < ActiveRecord::Migration[7.0]
  def change
    create_table :projects do |t|
      t.string :name, null: false
      t.string :slug, null: false
      t.string :github_name, null: false
      t.string :devpost_link
      t.string :project_link
      t.string :image
      t.boolean :featured, default: false
      # t.string :skill_ids, array: true, default: []

      t.timestamps
    end
  end
end
