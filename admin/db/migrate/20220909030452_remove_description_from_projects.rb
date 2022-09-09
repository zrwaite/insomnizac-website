class RemoveDescriptionFromProjects < ActiveRecord::Migration[7.0]
  def change
    remove_column :projects, :description, :string
  end
end
