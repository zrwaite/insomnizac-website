class AddSkillIdsToProjects < ActiveRecord::Migration[7.0]
  def change
    add_column :projects, :skill_ids, :string, array: true #, default: []
  end
end
